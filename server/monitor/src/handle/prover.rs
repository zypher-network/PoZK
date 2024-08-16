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
    docker_proxy_prefix: Option<String>,
    chain_id: U256,
}

impl ProverService {
    pub fn new(
        eth_cli: Provider<Http>,
        db: Arc<ReDB>,
        docker_manager: DockerManager,
        monitor_receiver: UnboundedReceiver<MonitorChanData>,
        miner: Address,
        docker_proxy_prefix: Option<String>,
        chain_id: U256,
    ) -> Result<Self> {
        Ok(Self {
            eth_cli,
            db,
            docker_manager,
            miner,
            monitor_receiver,
            docker_proxy_prefix,
            chain_id,
        })
    }

    pub fn run(mut self) {
        spawn(async move {
            while let Some(monitor_data) = self.monitor_receiver.recv().await {
                match monitor_data.event_type {
                    EventType::ApproveProver => {
                        let prover_address = {
                            let Some(prover) = monitor_data.data.get("prover") else {
                                log::warn!("[prover_service] not match prover");
                                continue;
                            };

                            let Some(prover_address) = prover.clone().into_address() else {
                                log::warn!("[prover_service] prover: {prover:?} not address type");
                                continue;
                            };

                            prover_address
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

                        let name = {
                            let (tx_data, func) = match utils::gen_prover_name_data(prover_address)
                            {
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

                        let repository = if let Some(proxy_prefix) = &self.docker_proxy_prefix {
                            format!("{proxy_prefix}/{DOCKER_HUB_REPO}/{prover_address:?}")
                        } else {
                            format!("{DOCKER_HUB_REPO}/{prover_address:?}")
                        };

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
