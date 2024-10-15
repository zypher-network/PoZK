use anyhow::{anyhow, Result};
use ethers::prelude::*;
use pozk_utils::{new_providers, new_signer, zero_gas, DefaultSigner, Stake, Task};
use std::sync::Arc;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::MonitorConfig;

const GAS_PRICE: u64 = 1_000_000_000; // 1 GWEI
const EXTRA_GAS: u64 = 10; // extra 10%

pub enum PoolMessage {
    ChangeController(LocalWallet),
    AcceptTask(u64),
    SubmitTask(u64, Vec<u8>),
    SubmitMinerTest(u64, Vec<u8>),
}

pub struct Pool {
    wallet: LocalWallet,
    task: Task<DefaultSigner>,
    stake: Stake<DefaultSigner>,
    miner: Address,
    zero_gas: Option<String>,
}

impl Pool {
    pub async fn new(cfg: &MonitorConfig, wallet: LocalWallet) -> Result<Self> {
        let providers = new_providers(&cfg.endpoints());
        if providers.is_empty() {
            return Err(anyhow!("No providers"));
        }

        let (task_address, _start) = cfg.task_address()?;
        let (stake_address, _start) = cfg.stake_address()?;

        let signer = new_signer(providers[0].clone(), wallet.clone()).await?;
        let task = Task::new(task_address, signer.clone());
        let stake = Stake::new(stake_address, signer);

        let miner = cfg.miner()?;
        let zero_gas = cfg.zero_gas.clone();

        Ok(Self {
            wallet,
            task,
            stake,
            miner,
            zero_gas,
        })
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
                    let stake_address = self.stake.address();

                    let signer = Arc::new(self.task.client_ref().with_signer(wallet.clone()));
                    self.task = Task::new(task_address, signer.clone());
                    self.stake = Stake::new(stake_address, signer);
                    self.wallet = wallet;
                }
                PoolMessage::AcceptTask(tid) => {
                    let gas_price = self
                        .task
                        .client_ref()
                        .get_gas_price()
                        .await
                        .unwrap_or(GAS_PRICE.into());
                    let extra_gas = gas_price + gas_price / U256::from(EXTRA_GAS);

                    let func = self
                        .task
                        .accept(U256::from(tid), self.miner)
                        .gas_price(extra_gas);
                    self.send(func).await;
                }
                PoolMessage::SubmitTask(tid, proof) => {
                    let gas_price = self
                        .task
                        .client_ref()
                        .get_gas_price()
                        .await
                        .unwrap_or(GAS_PRICE.into());
                    let extra_gas = gas_price + gas_price / U256::from(EXTRA_GAS);

                    let func = self
                        .task
                        .submit(U256::from(tid), proof.into())
                        .gas_price(extra_gas);
                    self.send(func).await;
                }
                PoolMessage::SubmitMinerTest(tid, proof) => {
                    let gas_price = self
                        .stake
                        .client_ref()
                        .get_gas_price()
                        .await
                        .unwrap_or(GAS_PRICE.into());
                    let extra_gas = gas_price + gas_price / U256::from(EXTRA_GAS);

                    let func = self
                        .stake
                        .miner_test_submit(U256::from(tid), true, proof.into())
                        .gas_price(extra_gas);
                    self.send(func).await;
                }
            }
        }
    }

    async fn send(&self, func: FunctionCall<Arc<DefaultSigner>, DefaultSigner, ()>) {
        if let Some(proxy) = &self.zero_gas {
            let _ = zero_gas(proxy, func.tx, &self.wallet).await;
        } else {
            match func.send().await {
                Ok(pending) => {
                    if let Ok(receipt) = pending.await {
                        info!(
                            "[Pool] Tx submitted, Gas used: {:?}",
                            receipt.map(|x| x.cumulative_gas_used)
                        );
                    } else {
                        error!("[Pool] Tx submit failed");
                    }
                }
                Err(err) => {
                    if let Some(rcode) = err.decode_revert::<String>() {
                        error!("[Pool] Tx failed: {}", rcode);
                    } else {
                        error!("[Pool] Tx failed: {}", err);
                    }
                }
            }
        }
    }
}
