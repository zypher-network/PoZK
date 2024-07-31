use crate::tx::{FuncType, TxChanData};
use anyhow::Result;
use docker::{ContainerNewOption, DockerManager, Volumes};
use ethers::abi::{Bytes, Token, Uint};
use ethers::utils::hex::hex::encode;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::future::Future;
use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::{fs, spawn};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum TaskType {
    RunTask,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TaskChanData {
    pub ty: TaskType,
    pub data: Vec<u8>,
    pub repo: String,
    pub tag: String,
    pub tid: Uint,
}

pub struct TaskService {
    docker_manager: DockerManager,
    task_receiver: UnboundedReceiver<TaskChanData>,
    tx_sender: UnboundedSender<TxChanData>,
    base_path: PathBuf,
}

impl TaskService {
    pub fn new(
        docker_manager: DockerManager,
        task_receiver: UnboundedReceiver<TaskChanData>,
        tx_sender: UnboundedSender<TxChanData>,
        bath_path: &str,
    ) -> Result<Self> {
        let base_path = PathBuf::from(bath_path);
        Ok(Self {
            docker_manager,
            task_receiver,
            tx_sender,
            base_path,
        })
    }

    pub fn run_task(
        data: TaskChanData,
        base_path: PathBuf,
        docker_manager: DockerManager,
        tx_sender: UnboundedSender<TxChanData>,
    ) {
        spawn(async move {
            let mut base_path = base_path;
            let base_str = format!("{}:{}", data.repo, data.tag);
            // - mkdir folder
            base_path.push(&base_str);
            match tokio::fs::create_dir(&base_path).await {
                Ok(_) => {}
                Err(e) => {
                    log::error!("data: {data:?}, create dir: {e:?}");
                    return;
                }
            }

            // - mkdir file
            let (input_file, input_base, publics_file, publics_path, proof_file, proof_path) = {
                let (input_file, input_base) = {
                    let input_file = format!("{}.input", &base_str);
                    let mut base_path = base_path.clone();
                    base_path.push(&input_file);
                    let input_data = format!("0x{}", encode(&data.data));
                    match tokio::fs::write(base_path.as_path(), input_data).await {
                        Ok(_) => {}
                        Err(e) => {
                            log::error!("data: {data:?}, create input file: {e:?}");
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
                            log::error!("data: {data:?}, create publics file: {e:?}");
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
                            log::error!("data: {data:?}, create proof file: {e:?}");
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
            {
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

                let ccf = match docker_manager
                    .new_container(&data.repo, &data.tag, &op)
                    .await
                {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("data: {data:?}, new container: {e:?}");
                        return;
                    }
                };

                match docker_manager.start_container(&ccf.id).await {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("data: {data:?}, start container: {e:?}");
                    }
                }
            }

            // - query output file
            let (publics, proof) = {
                let mut count = 0;
                let max_count = 20;

                let mut publics_res = None;
                let mut proof_res = None;

                loop {
                    if count >= max_count {
                        break;
                    }

                    let publics = match fs::read_to_string(&publics_path).await {
                        Ok(v) => v,
                        Err(e) => {
                            log::error!("data: {data:?}, read publics: {e:?}");
                            return;
                        }
                    };

                    let proof = match fs::read_to_string(&proof_path).await {
                        Ok(v) => v,
                        Err(e) => {
                            log::error!("data: {data:?}, read proof: {e:?}");
                            return;
                        }
                    };

                    if publics.is_empty() || proof.is_empty() {
                        count += 1;
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        continue;
                    }

                    publics_res.replace(publics);
                    proof_res.replace(proof);
                }

                if publics_res.is_none() || proof_res.is_none() {
                    log::warn!("data: {data:?}, get prover result is nil, retry: {count}");
                    return;
                }

                (publics_res.unwrap(), proof_res.unwrap())
            };

            // - send tx
            {
                let mut map = BTreeMap::new();
                let proof = Token::Bytes(Bytes::from(proof));
                let publics = Token::Bytes(Bytes::from(publics));

                map.insert("proof".to_string(), proof);
                map.insert("publics".to_string(), publics);

                let tx_chan_data = TxChanData {
                    ty: FuncType::Submit,
                    data: map,
                };

                match tx_sender.send(tx_chan_data) {
                    Ok(_) => {
                        log::info!("data: {data:?}, send to tx chan success")
                    }
                    Err(e) => {
                        log::error!("data: {data:?}, send to tx chan: {e:?}")
                    }
                }
            }
        });
    }

    pub fn run(mut self) {
        spawn(async move {
            while let Some(data) = self.task_receiver.recv().await {
                let base_path = self.base_path.clone();

                match data.ty {
                    TaskType::RunTask => {
                        Self::run_task(
                            data,
                            base_path,
                            self.docker_manager.clone(),
                            self.tx_sender.clone(),
                        );
                    }
                }
            }
        });
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    #[test]
    fn test() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let base_path = PathBuf::from("./test");
            tokio::fs::write(&base_path, b"").await.unwrap();
            let file = tokio::fs::read_to_string(base_path).await.unwrap();
            println!("content len: {}", file.len());
        });
    }
}
