use crate::event::EventType;
use crate::monitor::MonitorChanData;
use crate::utils;
use anyhow::Result;
use db::{ControllerKey, ReDB};
use docker::DockerManager;
use ethers::addressbook::Address;
use ethers::prelude::{Http, Middleware, Provider, U256};
use std::sync::Arc;
use tokio::spawn;
use tokio::sync::mpsc::UnboundedReceiver;

static DOCKER_HUB_REPO: &str = "zyphernetwork";

pub struct ProverService {
    eth_cli: Provider<Http>,
    db: Arc<ReDB>,
    docker_manager: DockerManager,
    miner: Address,
    monitor_receiver: UnboundedReceiver<MonitorChanData>,
    docker_proxy: Option<String>,
    chain_id: U256,
}

impl ProverService {
    pub fn new(
        eth_cli: Provider<Http>,
        db: Arc<ReDB>,
        docker_manager: DockerManager,
        monitor_receiver: UnboundedReceiver<MonitorChanData>,
        miner: Address,
        docker_proxy: Option<String>,
        chain_id: U256,
    ) -> Result<Self> {
        Ok(Self {
            eth_cli,
            db,
            docker_manager,
            miner,
            monitor_receiver,
            docker_proxy,
            chain_id,
        })
    }

    pub fn run(mut self) {
        spawn(async move {
            while let Some(monitor_data) = self.monitor_receiver.recv().await {
                match monitor_data.event_type {
                    EventType::ApproveProver => {
                        let (prover_token, prover_address) = {
                            let Some(prover) = monitor_data.data.get("prover") else {
                                log::warn!("[prover_service] not match prover");
                                continue;
                            };

                            let Some(prover_address) = prover.clone().into_address() else {
                                log::warn!("[prover_service] prover: {prover:?} not address type");
                                continue;
                            };

                            (prover.clone(), prover_address)
                        };

                        let version = {
                            let Some(version) = monitor_data.data.get("version") else {
                                log::warn!("[prover_service] version not match");
                                continue;
                            };

                            let Some(version) = version.clone().into_uint() else {
                                log::warn!("[prover_service] version: {version:?} not uint type");
                                continue;
                            };

                            format!("v{}", version.as_u64())
                        };

                        let overtime = {
                            let Some(overtime) = monitor_data.data.get("overtime") else {
                                log::warn!("[prover_service] overtime not match");
                                continue;
                            };

                            let Some(overtime) = overtime.clone().into_uint() else {
                                log::warn!("[prover_service] overtime: {overtime:?} not uint type");
                                continue;
                            };

                            overtime.as_u64()
                        };

                        // query is miner
                        {
                            let (stake_address, tx_data, func) =
                                match utils::gen_is_miner_data(&prover_token, self.miner) {
                                    Ok(v) => v,
                                    Err(e) => {
                                        log::error!("[prover_service] gen is miner ts data: {e:?}");
                                        continue;
                                    }
                                };

                            let tx = match utils::gen_tx(
                                &self.eth_cli,
                                Some(tx_data),
                                Some(self.miner),
                                stake_address,
                                None,
                                self.chain_id,
                                None,
                            )
                            .await
                            {
                                Ok(v) => v,
                                Err(e) => {
                                    log::error!("[prover_service] gen tx: {e:?}");
                                    continue;
                                }
                            };

                            let res = match self.eth_cli.call(&tx, None).await {
                                Ok(v) => v,
                                Err(e) => {
                                    log::error!("[prover_service] IsMiner call: {e:?}");
                                    continue;
                                }
                            };

                            match func.decode_output(res.as_ref()) {
                                Ok(v) => {
                                    let Some(t) = v.get(0) else {
                                        log::warn!("[prover_service] IsMiner result decode list not index 0: {v:?}");
                                        continue;
                                    };

                                    let Some(is_miner) = t.clone().into_bool() else {
                                        log::warn!(
                                            "[prover_service] IsMiner result not match bool"
                                        );
                                        continue;
                                    };

                                    if !is_miner {
                                        log::warn!(
                                            "[prover_service] miner: {:?} not miner",
                                            self.miner
                                        );
                                        continue;
                                    }

                                    log::info!("[prover_service] miner: {:?} is miner", self.miner);
                                }
                                Err(e) => {
                                    log::error!(
                                        "[prover_service] decode IsMiner call result: {e:?}"
                                    );
                                    continue;
                                }
                            };
                        };

                        // get name
                        let name = {
                            let (tx_data, func) = match utils::gen_prover_name_data() {
                                Ok(v) => v,
                                Err(e) => {
                                    log::error!("[prover_service] gen prover name tx data: {e:?}");
                                    continue;
                                }
                            };

                            let tx = match utils::gen_tx(
                                &self.eth_cli,
                                Some(tx_data),
                                Some(self.miner),
                                prover_address,
                                None,
                                self.chain_id,
                                None,
                            )
                            .await
                            {
                                Ok(v) => v,
                                Err(e) => {
                                    log::error!("[prover_service] gen prover name tx: {e:?}");
                                    return;
                                }
                            };

                            let res = match self.eth_cli.call(&tx, None).await {
                                Ok(v) => v,
                                Err(e) => {
                                    log::error!("[prover_service] call prover name: {e:?}");
                                    continue;
                                }
                            };

                            match func.decode_output(res.as_ref()) {
                                Ok(v) => {
                                    let Some(t) = v.get(0) else {
                                        log::warn!("[prover_service] ProverName result decode list not index 0: {v:?}");
                                        continue;
                                    };

                                    let Some(name) = t.clone().into_string() else {
                                        log::warn!(
                                            "[prover_service] ProverName result not match string"
                                        );
                                        continue;
                                    };

                                    name
                                }
                                Err(e) => {
                                    log::error!("[prover_service] ProverName decode: {e:?}");
                                    continue;
                                }
                            }
                        };

                        let repository = if let Some(proxy_prefix) = &self.docker_proxy {
                            format!("{proxy_prefix}/{DOCKER_HUB_REPO}/{prover_address:?}")
                        } else {
                            format!("{DOCKER_HUB_REPO}/{prover_address:?}")
                        };

                        // pull images
                        if let Err(e) = self.docker_manager.pull_image(&repository, &version).await
                        {
                            log::error!("[prover_service] pull image: {e:?}");
                            continue;
                        }

                        let image_info = match self
                            .docker_manager
                            .get_image_by_repository(&repository, &version)
                            .await
                        {
                            Ok(v) => v,
                            Err(e) => {
                                log::error!("[prover_service] get image: {e:?}");
                                continue;
                            }
                        };

                        let Some(image_info) = image_info else {
                            log::warn!("[prover_service] get image is null");
                            continue;
                        };

                        if let Err(e) = self.db.prover_add(
                            &ControllerKey(self.miner),
                            &image_info.id,
                            &name,
                            &image_info.repository,
                            &prover_address,
                            &image_info.created,
                            &image_info.tag,
                            overtime,
                            None,
                        ) {
                            log::error!("[prover_service] db prover add: {e:?}");
                            continue;
                        }

                        log::info!("[prover_service] prover update prover: {prover_address:?}, tag: {version:?}")
                    }
                    _ => continue,
                }
            }
        });
    }
}

#[cfg(test)]
mod test {
    use crate::{utils, Monitor, MonitorConfig, ProverService};
    use db::ReDB;
    use docker::DockerManager;
    use ethers::prelude::{Middleware, Provider, ProviderExt};
    use ethers::types::Address;
    use std::path::PathBuf;
    use std::str::FromStr;
    use std::sync::Arc;

    #[test]
    fn test_prover_service() {
        env_logger::init();
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(async {
            let config = MonitorConfig {
                task_address: "0x27DE7777C1c643B7F3151F7e4Bd3ba5dacc62793".to_string(),
                prover_address: "0x1c23e9F06b10f491e86b506c025080C96513C9f5".to_string(),
                stake_address: "0x003C1F8F552EE2463e517FDD464B929F8C0bFF06".to_string(),
                from: 36537285,
                delay_sec: 0,
                step: 1,
                wait_time: 0,
                block_number_type: "latest".to_string(),
                miner: "0x28B9FEAE1f3d76565AAdec86E7401E815377D9Cc".to_string(),
                docker_proxy: Some("docker.registry.cyou".to_string()),
            };

            let task_address =
                Address::from_str("0x27DE7777C1c643B7F3151F7e4Bd3ba5dacc62793").unwrap();
            let prover_address =
                Address::from_str("0x1c23e9F06b10f491e86b506c025080C96513C9f5").unwrap();
            let stake_address =
                Address::from_str("0x003C1F8F552EE2463e517FDD464B929F8C0bFF06").unwrap();

            let miner = Address::from_str("0x28B9FEAE1f3d76565AAdec86E7401E815377D9Cc").unwrap();

            let db_path = PathBuf::from("/home/cloud/tmp/pozk");
            let db = Arc::new(ReDB::new(&db_path, false).unwrap());

            let docker_manager = DockerManager::new().unwrap();

            let eth_cli = Provider::connect("https://opbnb-testnet-rpc.bnbchain.org").await;
            let chain_id = eth_cli.get_chainid().await.unwrap();

            utils::init_functions(task_address, stake_address, prover_address).unwrap();

            let mut monitor = Monitor::new(&config, eth_cli.clone()).await.unwrap();

            let prover_service = ProverService::new(
                eth_cli.clone(),
                db.clone(),
                docker_manager.clone(),
                monitor.register(),
                miner,
                Some("docker.registry.cyou".to_string()),
                chain_id,
            )
            .unwrap();

            prover_service.run();
            monitor.run();

            tokio::signal::ctrl_c().await.unwrap();
        });
    }
}
