use crate::event::EventType;
use crate::handle::tx::TxChanData;
use crate::monitor::MonitorChanData;
use crate::utils;
use crate::utils::FuncType;
use anyhow::Result;
use chrono::Utc;
use db::{ControllerKey, ReDB};
use docker::{ContainerNewOption, DockerManager, Volumes};
use ethers::abi::{Token, Uint};
use ethers::addressbook::Address;
use ethers::prelude::{Http, Middleware, Provider, U256};
use ethers::utils::hex;
use ethers::utils::hex::hex::encode;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
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
    prover_host_path: PathBuf,
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
        prover_host_path: &str,
        chain_id: U256,
        miner: Address,
    ) -> Result<Self> {
        let base_path = PathBuf::from(base_path);
        let prover_host_path = PathBuf::from(prover_host_path);

        Ok(Self {
            db,
            docker_manager,
            eth_cli,
            chain_id,
            monitor_receiver,
            tx_sender,
            base_path,
            prover_host_path,
            miner,
        })
    }

    pub fn run_task(
        eth_cli: Provider<Http>,
        data: RunTaskData,
        base_path: PathBuf,
        prover_host_path: PathBuf,
        docker_manager: DockerManager,
        tx_sender: UnboundedSender<TxChanData>,
        db: Arc<ReDB>,
        chain_id: U256,
    ) {
        spawn(async move {
            let start_time = Utc::now().timestamp();

            let mut prover_host_path = prover_host_path;

            let mut base_path = base_path;
            let base_str = format!("{:?}-{}-{}", data.prover, data.tag, data.tid.to_string());

            // - create folder
            base_path.push(&base_str);
            prover_host_path.push(&base_str);
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
            let container_id = {
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
                        host_volumes: prover_host_path.as_os_str().to_str().unwrap().to_string(), // safe
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
                    .new_container(&meta.repository, &data.tag, data.tid.as_u64(), &op)
                    .await
                {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("[run_task] new container: {e:?}");
                        return;
                    }
                };

                let Some(container_id) = ccf.id else {
                    log::warn!("[run_task] container_id is nil");
                    return;
                };

                // insert container_id to db
                match db.prover_container_add(&data.miner, &data.prover, &data.tag, &container_id) {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("[run_task] add container to db fail: {e:?}");
                    }
                }

                match docker_manager.start_container(&container_id).await {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("[run_task] start container: {e:?}");
                    }
                }

                container_id
            };

            // delete container
            Self::delete_container(
                docker_manager.clone(),
                db.clone(),
                container_id.clone(),
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
                        Ok(v) => {
                            if !v.is_empty() {
                                publics_res.replace(v);
                            }
                        }
                        Err(e) => {
                            log::error!("[run_task] read publics: {e:?}");
                            return;
                        }
                    };

                    match fs::read_to_string(&proof_path).await {
                        Ok(v) => {
                            if !v.is_empty() {
                                proof_res.replace(v);
                            }
                        }
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
                let proof = match hex::decode(proof) {
                    Ok(v) => Token::Bytes(v),
                    Err(e) => {
                        log::error!("[run_task] decode proof: {e:?}");
                        return;
                    }
                };

                let publics = match hex::decode(publics) {
                    Ok(v) => Token::Bytes(v),
                    Err(e) => {
                        log::error!("[run_task] decode proof: {e:?}");
                        return;
                    }
                };

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
                    None,
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

                log::debug!("[run_task] Duration: [{}]sec", end_time - start_time);
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
                            let Some(cstate) = state else {
                                log::warn!("[delete_container] state is nil");
                                return;
                            };

                            if cstate.running.unwrap_or(false) {
                                tokio::time::sleep(Duration::from_secs(2)).await;
                                continue;
                            } else {
                                flag = true;
                            }
                        }
                        Err(e) => {
                            log::error!("[delete_container] query container status: {id}, err: {e:?}")
                        }
                    }
                }

                log::debug!(
                    "[delete_container] query container status: {id}, count: {count}, can delete: {flag}"
                );
                flag
            };

            // remove container
            if can_delete {
                match db.prover_container_remove(&data.miner, &data.prover, &data.tag, &id) {
                    Ok(_) => {
                        log::debug!("[delete_container] db remove container: {id}, success");
                    }
                    Err(e) => {
                        log::error!("[delete_container] remove container: {id}: {e:?}");
                        return;
                    }
                }
                match docker_manager.remove_container(&id).await {
                    Ok(_) => {
                        log::debug!("[delete_container] docker remove container: {id}, success");
                    }
                    Err(e) => {
                        log::error!("[delete_container] docker remove container: {id}: {e:?}");
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
                                None,
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
                                None,
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
                                None,
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
                                    self.prover_host_path.clone(),
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

#[cfg(test)]
mod test {
    use crate::utils;
    use ethers::abi::{Token, Uint};
    use ethers::addressbook::Address;
    use ethers::prelude::{Middleware, Provider, ProviderExt};
    use ethers::utils::hex::hex;
    use std::str::FromStr;

    #[test]
    fn test_task() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(async {

            let task_market_address = Address::from_str("0x27DE7777C1c643B7F3151F7e4Bd3ba5dacc62793").unwrap();
            let prover_market_address = Address::from_str("0x1c23e9F06b10f491e86b506c025080C96513C9f5").unwrap();
            let stake_address = Address::from_str("0x003C1F8F552EE2463e517FDD464B929F8C0bFF06").unwrap();

            utils::init_functions(
                task_market_address,
                stake_address,
                prover_market_address,
            ).unwrap();

            let controller = Address::from_str("0x29474bEAc49D74099e995b351CA1eDc59cE5bBAb").unwrap();

            let proof = "092f39dff9b9a9db202a7eee348cd1f9875c7b42cdce24ed5e30c5c2949a64d32fbc41193251e707f8319e87d61fc49577c36d367ab0c8814daaf5c175a5685b0484c8dbc13dbdca93b4b486a2ace8397da092ceb3b6c7f78b43215575cbf32227cfaa2d0d056a09222aca421953ae79118d5b18fc2a36b45b9092cd667238020db385cd42f39d924a62a79cd2f6dd393f0ab7581595b032c6ec8ec599d7898d02e53eefa6f30b198412566ed23b9ef5d7a39e3fc20c76e636ff7e2c1f415ba82efa582c6a79722a1e4d9e372af6700972c639b61f6ef6b9f99ceadc3963f570042ff5ebf5b4068fe61252c38b3ff27519c28714cca6f5f6c5a10ddb9687a4d7";
            let publics = "0000000000000000000000000000000000000000000000000000200800000000000000000000000000000000000000000000000000000088600444000050002300000000000000000000000000000000003c0cf3cc8f230c8f0cf3ff0ef3c3330000000000000000000000000000000000000000000000000000000000001a850000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003c00000000000000000000000000000000000000000000000000000000000001c8";
            let tid  = 92;

            let t_publics = Token::Bytes(hex::decode(publics).unwrap());
            let t_proof = Token::Bytes(hex::decode(proof).unwrap());

            let (task_market_address,tx_data) = utils::gen_submit_data(Token::Uint(Uint::from(tid)), t_publics, t_proof).unwrap();

            let eth_cli = Provider::connect("https://opbnb-testnet-rpc.bnbchain.org").await;
            let chain_id = eth_cli.get_chainid().await.unwrap();

            let tx = utils::gen_tx(
                &eth_cli,
                Some(tx_data),
                Some(controller),
                task_market_address,
                None,
                chain_id,
                None,
            )
                .await.unwrap();

            println!("tx: {tx:?}");
        });
    }
}
