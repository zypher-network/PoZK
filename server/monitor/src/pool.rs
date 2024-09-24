use anyhow::{anyhow, Result};
use ethers::prelude::*;
use pozk_utils::{new_providers, new_signer, DefaultSigner, Task};
use std::sync::Arc;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::MonitorConfig;

const GAS_PRICE: u64 = 1_000_000_000; // 1 GWEI
const EXTRA_GAS: u64 = 10; // extra 10%

// TODO use 0-gas service

pub enum PoolMessage {
    ChangeController(LocalWallet),
    AcceptTask(u64),
    SubmitTask(u64, Vec<u8>, Vec<u8>),
}

pub struct Pool {
    task: Task<DefaultSigner>,
    miner: Address,
}

impl Pool {
    pub async fn new(cfg: &MonitorConfig, wallet: LocalWallet) -> Result<Self> {
        let providers = new_providers(&cfg.endpoints());
        if providers.is_empty() {
            return Err(anyhow!("No providers"));
        }

        let (task_address, _start) = cfg.task_address()?;
        let signer = new_signer(providers[0].clone(), wallet).await?;
        let task = Task::new(task_address, signer);

        let miner = cfg.miner()?;

        Ok(Self { task, miner })
    }

    pub fn run(self) -> UnboundedSender<PoolMessage> {
        let (sender, receiver) = unbounded_channel();
        tokio::spawn(self.listen(receiver));
        sender
    }

    async fn listen(mut self, mut receiver: UnboundedReceiver<PoolMessage>) {
        while let Some(msg) = receiver.recv().await {
            match msg {
                PoolMessage::ChangeController(wallet) => {
                    // change controller account
                    let task_address = self.task.address();
                    let signer = Arc::new(self.task.client_ref().with_signer(wallet));
                    self.task = Task::new(task_address, signer);
                }
                PoolMessage::AcceptTask(tid) => {
                    let gas_price = self
                        .task
                        .client_ref()
                        .get_gas_price()
                        .await
                        .unwrap_or(GAS_PRICE.into());
                    let extra_gas = gas_price + gas_price / U256::from(EXTRA_GAS);

                    match self
                        .task
                        .accept(U256::from(tid), self.miner)
                        .gas_price(extra_gas)
                        .send()
                        .await
                    {
                        Ok(pending) => {
                            if let Ok(receipt) = pending.await {
                                info!(
                                    "Accepted, Gas used: {:?}",
                                    receipt.map(|x| x.cumulative_gas_used)
                                );
                            } else {
                                error!("Accept failed");
                            }
                        }
                        Err(err) => {
                            if let Some(rcode) = err.decode_revert::<String>() {
                                error!("Accept failed: {}", rcode);
                            } else {
                                error!("Accept failed: {}", err);
                            }
                        }
                    }
                }
                PoolMessage::SubmitTask(tid, publics, proof) => {
                    let gas_price = self
                        .task
                        .client_ref()
                        .get_gas_price()
                        .await
                        .unwrap_or(GAS_PRICE.into());
                    let extra_gas = gas_price + gas_price / U256::from(EXTRA_GAS);

                    match self
                        .task
                        .submit(U256::from(tid), publics.into(), proof.into())
                        .gas_price(extra_gas)
                        .send()
                        .await
                    {
                        Ok(pending) => {
                            if let Ok(receipt) = pending.await {
                                info!(
                                    "Submitted, Gas used: {:?}",
                                    receipt.map(|x| x.cumulative_gas_used)
                                );
                            } else {
                                error!("Submit failed");
                            }
                        }
                        Err(err) => {
                            if let Some(rcode) = err.decode_revert::<String>() {
                                error!("Submit failed: {}", rcode);
                            } else {
                                error!("submit failed: {}", err);
                            }
                        }
                    }
                }
            }
        }
    }
}
