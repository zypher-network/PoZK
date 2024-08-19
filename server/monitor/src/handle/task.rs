use crate::event::EventType;
use crate::handle::tx::TxChanData;
use crate::monitor::MonitorChanData;
use crate::utils;
use crate::utils::FuncType;
use anyhow::Result;
use db::{ControllerKey, ReDB};
use docker::{ContainerNewOption, DockerManager, Volumes};
use ethers::abi::{Token, Uint};
use ethers::addressbook::Address;
use ethers::prelude::{Http, Middleware, Provider, U256};
use ethers::utils::hex::hex::encode;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use chrono::Utc;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::{fs, spawn};

pub struct TaskService {
    db: Arc<ReDB>,
    docker_manager: DockerManager,
    eth_cli: Provider<Http>,
    chain_id: U256,
    monitor_receiver: UnboundedReceiver<MonitorChanData>,
    tx_sender: UnboundedSender<TxChanData>,
    base_path: PathBuf,
    miner: Address,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RunTaskData {
    pub data: Vec<u8>,
    pub prover: Address,
    pub tag: String,
    pub tid: Uint,
    pub miner: ControllerKey,
    pub controller: ControllerKey,
}

impl TaskService {
    pub fn new(
        eth_cli: Provider<Http>,
        db: Arc<ReDB>,
        docker_manager: DockerManager,
        monitor_receiver: UnboundedReceiver<MonitorChanData>,
        tx_sender: UnboundedSender<TxChanData>,
        base_path: &str,
        chain_id: U256,
        miner: Address,
    ) -> Result<Self> {
        let base_path = PathBuf::from(base_path);
        Ok(Self {
            db,
            docker_manager,
            eth_cli,
            chain_id,
            monitor_receiver,
            tx_sender,
            base_path,
            miner,
        })
    }

    pub fn run_task(
        eth_cli: Provider<Http>,
        data: RunTaskData,
        base_path: PathBuf,
        docker_manager: DockerManager,
        tx_sender: UnboundedSender<TxChanData>,
        db: Arc<ReDB>,
        chain_id: U256,
    ) {
        spawn(async move {
            let start_time = Utc::now().timestamp();

            let mut base_path = base_path;
            let base_str = format!("{:?}-{}-{}", data.prover, data.tag, data.tid.to_string());

            // - create folder
            base_path.push(&base_str);
            match tokio::fs::create_dir(&base_path).await {
                Ok(_) => {}
                Err(e) => {
                    log::error!("[run_task] create dir: {e:?}");
                    return;
                }
            }

            // - create file
            let (input_file, _input_base, publics_file, publics_path, proof_file, proof_path) = {
                let (input_file, input_base) = {
                    let input_file = format!("{}.input", &base_str);
                    let mut base_path = base_path.clone();
                    base_path.push(&input_file);
                    let input_data = format!("0x{}", encode(&data.data));
                    match tokio::fs::write(base_path.as_path(), input_data).await {
                        Ok(_) => {}
                        Err(e) => {
                            log::error!("[run_task] create input file: {e:?}");
                            return;
                        }
                    }
                    (input_file, base_path)
                };

                let (publics_file, publics_path) = {
                    let publics_file = format!("{}.publics", &base_str);
                    let mut base_path = base_path.clone();
                    base_path.push(&publics_file);
                    match tokio::fs::write(base_path.as_path(), b"").await {
                        Ok(_) => {}
                        Err(e) => {
                            log::error!("[run_task] create publics file: {e:?}");
                            return;
                        }
                    }
                    (publics_file, base_path)
                };

                let (proof_file, proof_path) = {
                    let proof_file = format!("{}.proof", base_str);
                    let mut base_path = base_path.clone();
                    base_path.push(&proof_file);
                    match tokio::fs::write(base_path.as_path(), b"").await {
                        Ok(_) => {}
                        Err(e) => {
                            log::error!("[run_task] create proof file: {e:?}");
                            return;
                        }
                    }
                    (proof_file, base_path)
                };

                (
                    input_file,
                    input_base,
                    publics_file,
                    publics_path,
                    proof_file,
                    proof_path,
                )
            };

            // - run task
            let ccf = {
                let op = ContainerNewOption {
                    cpu_shares: None,
                    cpus: None,
                    env: Some(vec![
                        format!("INPUT=/data/{input_file}"),
                        format!("OUTPUT=/data/{publics_file}"),
                        format!("PROOF=/data/{proof_file}"),
                    ]),
                    cmd: None,
                    expose: None,
                    memory: None,
                    volumes: Some(vec![Volumes {
                        src_volumes: "/data".to_string(),
                        host_volumes: base_path.as_os_str().to_str().unwrap().to_string(), // safe
                    }]),
                };

                let meta = match db.prover_meta(&data.miner, &data.prover, &data.tag) {
                    Ok(v) => {
                        let Some(meta) = v else {
                            log::warn!("[run_task] get prover meta nil");
                            return;
                        };
                        meta
                    }
                    Err(e) => {
                        log::error!("[run_task] get prover meta: {e:?}");
                        return;
                    }
                };

                let ccf = match docker_manager
                    .new_container(&meta.repository, &data.tag, &op)
                    .await
                {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("[run_task] new container: {e:?}");
                        return;
                    }
                };

                // insert container_id to db
                match db.prover_container_add(&data.miner, &data.prover, &data.tag, &ccf.id) {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("[run_task] add container to db fail: {e:?}");
                    }
                }

                match docker_manager.start_container(&ccf.id).await {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("[run_task] start container: {e:?}");
                    }
                }

                ccf
            };

            // delete container
            Self::delete_container(
                docker_manager.clone(),
                db.clone(),
                ccf.id.clone(),
                data.clone(),
            );

            // - query output file
            let (publics, proof) = {
                let mut count = 0;
                let max_count = 20;

                let mut publics_res = None;
                let mut proof_res = None;

                while publics_res.is_none() || proof_res.is_none() {
                    count += 1;

                    if count >= max_count {
                        break;
                    }

                    match fs::read_to_string(&publics_path).await {
                        Ok(v) => publics_res.replace(v),
                        Err(e) => {
                            log::error!("[run_task] read publics: {e:?}");
                            return;
                        }
                    };

                    match fs::read_to_string(&proof_path).await {
                        Ok(v) => proof_res.replace(v),
                        Err(e) => {
                            log::error!("[run_task] read proof: {e:?}");
                            return;
                        }
                    };

                    tokio::time::sleep(Duration::from_secs(2)).await;
                }

                if publics_res.is_none() || proof_res.is_none() {
                    log::warn!("[run_task] get prover result is nil, retry: {count}");
                    return;
                }

                (publics_res.unwrap(), proof_res.unwrap())
            };

            // - send tx
            {
                let proof = Token::Bytes(ethers::abi::Bytes::from(proof));
                let publics = Token::Bytes(ethers::abi::Bytes::from(publics));

                let (task_market_address, tx_data) =
                    match utils::gen_submit_data(Token::Uint(data.tid), publics, proof) {
                        Ok(v) => v,
                        Err(e) => {
                            log::error!("[run_task] gen submit tx data: {e:?}");
                            return;
                        }
                    };

                let tx = match utils::gen_tx(
                    &eth_cli,
                    Some(tx_data),
                    Some(data.controller.0),
                    task_market_address,
                    None,
                    chain_id,
                )
                .await
                {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("[run_task] gen submit tx: {e:?}");
                        return;
                    }
                };

                match tx_sender.send(TxChanData {
                    sync: None,
                    tx,
                    func_type: FuncType::Submit,
                }) {
                    Ok(_) => {
                        log::debug!("[run_task] send to tx chan success")
                    }
                    Err(e) => {
                        log::error!("[run_task] send to tx chan: {e:?}")
                    }
                }

                let end_time = Utc::now().timestamp();

                log::debug!(
                    "[run_task] Duration: [{}]sec",
                    end_time - start_time
                );
            }
        });
    }

    pub fn delete_container(
        docker_manager: DockerManager,
        db: Arc<ReDB>,
        id: String,
        data: RunTaskData,
    ) {
        spawn(async move {
            // query container status util to not running
            let can_delete = {
                let mut count = 0;
                let max_count = 20;

                let mut flag = false;

                while !flag {
                    count += 1;

                    if count >= max_count {
                        break;
                    }

                    match docker_manager.query_container_status(&id).await {
                        Ok(state) => {
                            if state.running {
                                tokio::time::sleep(Duration::from_secs(2)).await;
                                continue;
                            } else {
                                flag = true;
                            }
                        }
                        Err(e) => {
                            log::error!("[run_task] query container status: {id}, err: {e:?}")
                        }
                    }
                }

                log::debug!(
                    "[run_task] query container status: {id}, count: {count}, can delete: {flag}"
                );
                flag
            };

            // remove container
            if can_delete {
                match db.prover_container_remove(&data.miner, &data.prover, &data.tag, &id) {
                    Ok(_) => {
                        log::debug!("[run_task] db remove container: {id}, success");
                    }
                    Err(e) => {
                        log::error!("[run_task] remove container: {id}: {e:?}");
                        return;
                    }
                }
                match docker_manager.remove_container(&id).await {
                    Ok(_) => {
                        log::debug!("[run_task] docker remove container: {id}, success");
                    }
                    Err(e) => {
                        log::error!("[run_task] docker remove container: {id}: {e:?}");
                    }
                }
            }
        });
    }

    pub fn run(mut self) {
        spawn(async move {
            let miner_key = ControllerKey(self.miner);

            while let Some(monitor_data) = self.monitor_receiver.recv().await {
                match monitor_data.event_type {
                    EventType::CreateTask => {
                        let ty = monitor_data.event_type;

                        let controller = {
                            let result = self.db.controller_set_entry(&miner_key);
                            match result {
                                Ok((key, _signing_key)) => key.0,
                                Err(e) => {
                                    log::error!(
                                        "[task_service] handle: {ty:?}, get controller set: {:?}",
                                        e
                                    );
                                    return;
                                }
                            }
                        };

                        let (prover_token, prover_address) = {
                            let Some(prover) = monitor_data.data.get("prover") else {
                                log::warn!("[task_service] handle: {ty:?}, not match prover");
                                continue;
                            };

                            let Some(prover_address) = prover.clone().into_address() else {
                                log::warn!("[task_service] handle: {ty:?}, prover: {prover:?} not address type");
                                continue;
                            };

                            (prover.clone(), prover_address)
                        };

                        let (id, tid) = {
                            let Some(id) = monitor_data.data.get("id") else {
                                log::warn!("[task_service] handle: {ty:?}, not match id");
                                continue;
                            };

                            let Some(tid) = id.clone().into_uint() else {
                                log::warn!("[task_service] handle: {ty:?}, id not uid type");
                                continue;
                            };

                            (id.clone(), tid)
                        };

                        let input = {
                            let Some(data) = monitor_data.data.get("data") else {
                                log::warn!("[task_service] handle: {ty:?}, not match data");
                                continue;
                            };
                            let Some(data) = data.clone().into_bytes() else {
                                log::warn!(
                                    "[task_service] handle: {ty:?}, data: {data:?} not bytes type"
                                );
                                continue;
                            };
                            data
                        };

                        // query is miner
                        let _ = {
                            let (stake_address, tx_data, func) = match utils::gen_is_miner_data(
                                &prover_token,
                                self.miner,
                            ) {
                                Ok(v) => v,
                                Err(e) => {
                                    log::error!("[task_service] handle: {ty:?}, gen is miner ts data: {e:?}");
                                    continue;
                                }
                            };

                            let tx = match utils::gen_tx(
                                &self.eth_cli,
                                Some(tx_data),
                                Some(controller),
                                stake_address,
                                None,
                                self.chain_id,
                            )
                            .await
                            {
                                Ok(v) => v,
                                Err(e) => {
                                    log::error!("[task_service] handle: {ty:?}, gen tx: {e:?}");
                                    continue;
                                }
                            };

                            let res = match self.eth_cli.call(&tx, None).await {
                                Ok(v) => v,
                                Err(e) => {
                                    log::error!(
                                        "[task_service] handle: {ty:?}, IsMiner call: {e:?}"
                                    );
                                    continue;
                                }
                            };

                            match func.decode_output(res.as_ref()) {
                                Ok(v) => {
                                    let Some(t) = v.get(0) else {
                                        log::warn!("[task_service] handle: {ty:?}, IsMiner result decode list not index 0: {v:?}");
                                        continue;
                                    };

                                    let Some(is_miner) = t.clone().into_bool() else {
                                        log::warn!(
                                            "[task_service] handle: {ty:?}, IsMiner result not match bool"
                                        );
                                        continue;
                                    };

                                    if !is_miner {
                                        log::warn!(
                                            "[task_service] handle: {ty:?}, miner: {:?} not miner",
                                            self.miner
                                        );
                                        continue;
                                    }

                                    log::debug!(
                                        "[task_service] handle: {ty:?}, miner: {:?} is miner",
                                        self.miner
                                    );
                                }
                                Err(e) => {
                                    log::error!(
                                        "[task_service] handle: {ty:?}, decode IsMiner call result: {e:?}"
                                    );
                                    continue;
                                }
                            };
                        };

                        let tag = {
                            let (prover_address, tx_data, func) =
                                match utils::gen_prover_version_data(&prover_token) {
                                    Ok(v) => v,
                                    Err(e) => {
                                        log::error!("[task_service] handle: {ty:?}, gen is miner ts data: {e:?}");
                                        continue;
                                    }
                                };

                            let tx = match utils::gen_tx(
                                &self.eth_cli,
                                Some(tx_data),
                                Some(controller),
                                prover_address,
                                None,
                                self.chain_id,
                            )
                            .await
                            {
                                Ok(v) => v,
                                Err(e) => {
                                    log::error!("[task_service] handle: {ty:?}, gen tx: {e:?}");
                                    continue;
                                }
                            };

                            let res = match self.eth_cli.call(&tx, None).await {
                                Ok(v) => v,
                                Err(e) => {
                                    log::error!(
                                        "[task_service] handle: {ty:?}, prover version call: {e:?}"
                                    );
                                    continue;
                                }
                            };

                            let version = match func.decode_output(res.as_ref()) {
                                Ok(v) => {
                                    let Some(t) = v.get(0) else {
                                        log::warn!(
                                                "[task_service] handle: {ty:?}, IsMiner result decode list not index 0: {v:?}"
                                            );
                                        continue;
                                    };

                                    let Some(version) = t.clone().into_uint() else {
                                        log::warn!("[task_service] handle: {ty:?}, version result not match uint");
                                        continue;
                                    };

                                    log::debug!(
                                        "[task_service] handle: {ty:?} version: {version:?} "
                                    );

                                    version.to_string()
                                }
                                Err(e) => {
                                    log::error!(
                                            "[task_service] handle: {ty:?}, get prover version : {prover_address:?} {e:?}"
                                        );
                                    continue;
                                }
                            };

                            format!("v{version}")
                        };

                        match self.db.prover_meta(&miner_key, &prover_address, &tag) {
                            Ok(v) => {
                                let Some(_v) = v else {
                                    log::warn!("[task_service] handle: {ty:?}, query prover: {prover_address:?}, tag: {tag}");
                                    continue;
                                };
                            }
                            Err(e) => {
                                log::error!("[task_service] handle: {ty:?}, query prover: {prover_address:?}, tag: {tag} err: {e:?}");
                                continue;
                            }
                        };

                        let accept_task_tx = {
                            let (task_market_address, tx_data, _func) =
                                match utils::gen_accept_task_data(id, self.miner) {
                                    Ok(v) => v,
                                    Err(e) => {
                                        log::error!("[task_service] handle: {ty:?}, gen accept task tx data: {e:?}");
                                        continue;
                                    }
                                };

                            match utils::gen_tx(
                                &self.eth_cli,
                                Some(tx_data),
                                Some(controller),
                                task_market_address,
                                None,
                                self.chain_id,
                            )
                            .await
                            {
                                Ok(v) => v,
                                Err(e) => {
                                    log::error!(
                                        "[task_service] handle: {ty:?}, gen accept task tx: {e:?}"
                                    );
                                    continue;
                                }
                            }
                        };

                        {
                            let (sender, mut receiver) = unbounded_channel();
                            if let Err(e) = self.tx_sender.send(TxChanData {
                                sync: Some(sender),
                                tx: accept_task_tx,
                                func_type: FuncType::AcceptTask,
                            }) {
                                log::error!("[task_service] handle: {ty:?}, send accept task tx to chan: {e:?}");
                                receiver.close();
                                continue;
                            }

                            while let Some(_receipt) = receiver.recv().await {
                                let run_task_data = RunTaskData {
                                    data: input.clone(),
                                    prover: prover_address,
                                    tag: tag.clone(),
                                    tid,
                                    miner: miner_key.clone(),
                                    controller: ControllerKey(controller),
                                };

                                log::debug!("[task_service] handle: {ty:?}, run task data: {run_task_data:?}");

                                Self::run_task(
                                    self.eth_cli.clone(),
                                    run_task_data,
                                    self.base_path.clone(),
                                    self.docker_manager.clone(),
                                    self.tx_sender.clone(),
                                    self.db.clone(),
                                    self.chain_id,
                                )
                            }
                        }
                    }
                    _ => continue,
                }
            }
        });
    }
}
