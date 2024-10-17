use anyhow::{anyhow, Result};
use async_recursion::async_recursion;
use ethers::prelude::*;
use pozk_utils::{
    check_zero_gas, create_zero_gas, new_providers, new_signer, zero_gas, AAWallet, Controller,
    DefaultProvider, DefaultSigner, Stake, Task,
};
use std::sync::Arc;
use tokio::{
    select,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    time::sleep,
};

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
    controller: Controller<DefaultProvider>,
    miner: Address,
    chain: u64,
    zero_gas: String,
    zero_gas_working: bool,
    zero_gas_wallet: AAWallet<DefaultSigner>,
    zero_gas_nonce: u64,
}

enum InnerFuture {
    Message(PoolMessage),
    ZeroGas,
}

impl Pool {
    pub async fn new(cfg: &MonitorConfig, wallet: LocalWallet, ready: bool) -> Result<Self> {
        let wallet_address = wallet.address();
        let providers = new_providers(&cfg.endpoints());
        if providers.is_empty() {
            return Err(anyhow!("No providers"));
        }
        let chain = providers[0].get_chainid().await?.as_u64();

        let (task_address, _start) = cfg.task_address()?;
        let (stake_address, _start) = cfg.stake_address()?;
        let (controller_address, _start) = cfg.controller_address()?;

        let signer = new_signer(providers[0].clone(), wallet.clone()).await?;
        let task = Task::new(task_address, signer.clone());
        let stake = Stake::new(stake_address, signer.clone());
        let controller = Controller::new(controller_address, providers[0].clone());

        let miner = cfg.miner()?;
        let zero_gas = cfg.zero_gas.clone();
        let zero_gas_wallet = AAWallet::new(Address::zero(), signer);

        let mut pool = Self {
            wallet,
            task,
            stake,
            miner,
            controller,
            chain,
            zero_gas,
            zero_gas_wallet,
            zero_gas_working: false,
            zero_gas_nonce: 0,
        };

        if ready {
            info!("[Pool] set controller to: {}", wallet_address);
            pool.check().await;
        }

        Ok(pool)
    }

    pub fn run(self) -> UnboundedSender<PoolMessage> {
        let (sender, receiver) = unbounded_channel();
        tokio::spawn(self.listen(receiver));
        sender
    }

    async fn listen(mut self, mut recv: UnboundedReceiver<PoolMessage>) {
        loop {
            let work = select! {
                w = async {
                    recv.recv().await.map(InnerFuture::Message)
                } => w,
                w = async {
                    sleep(std::time::Duration::from_secs(600)).await;
                    Some(InnerFuture::ZeroGas)
                } => w,
            };

            match work {
                Some(InnerFuture::Message(msg)) => self.handle(msg).await,
                Some(InnerFuture::ZeroGas) => self.check().await,
                None => break,
            }
        }
    }

    /// verify controller (or 0 gas wallet) is valid controller to miner
    async fn verify(&self, account: Address) -> bool {
        if let Ok(res) = self.controller.check(self.miner, account).await {
            res
        } else {
            false
        }
    }

    /// check zero gas is working or not
    async fn check(&mut self) {
        if !self.zero_gas.is_empty()
            && check_zero_gas(&self.zero_gas, self.wallet.address())
                .await
                .is_ok()
        {
            let aa = self.zero_gas_wallet.address();
            if aa == Address::zero() {
                // create zero gas wallet
                if let Ok(new_aa) = create_zero_gas(&self.zero_gas, self.wallet.address()).await {
                    info!("[Pool] 0 gas wallet fetched: {}", aa);
                    self.zero_gas_wallet = self.zero_gas_wallet.at(new_aa).into();
                    self.reset_nonce().await;

                    // check aa is valid controller
                    if self.verify(new_aa).await {
                        info!("[Pool] 0 gas wallet actived");
                        self.zero_gas_working = true;
                    } else {
                        warn!("[Pool] 0 gas wallet not set to controller");
                    }
                }
            } else {
                if !self.zero_gas_working {
                    if self.verify(aa).await {
                        self.zero_gas_working = true;
                    }
                }
            }
        } else {
            self.zero_gas_working = false;
        }
    }

    /// reset zero gas nonce, sync with chain
    async fn reset_nonce(&mut self) {
        if let Ok(nonce) = self.zero_gas_wallet.nonce().await {
            self.zero_gas_nonce = nonce.as_u64();
        }
    }

    async fn handle(&mut self, msg: PoolMessage) {
        match msg {
            PoolMessage::ChangeController(wallet) => {
                info!("[Pool] changed controller to: {}", wallet.address());

                // change controller account
                let task_address = self.task.address();
                let stake_address = self.stake.address();

                let signer = Arc::new(self.task.client_ref().with_signer(wallet.clone()));
                self.task = Task::new(task_address, signer.clone());
                self.stake = Stake::new(stake_address, signer);
                self.wallet = wallet;
                self.check().await;
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
                self.send(func, true).await;
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
                self.send(func, true).await;
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
                self.send(func, true).await;
            }
        }
    }

    #[async_recursion]
    async fn send(
        &mut self,
        func: FunctionCall<Arc<DefaultSigner>, DefaultSigner, ()>,
        reset: bool,
    ) {
        if self.zero_gas_working {
            match zero_gas(
                &self.zero_gas,
                func.tx.clone(),
                self.chain,
                self.zero_gas_wallet.address(),
                self.zero_gas_nonce,
                &self.wallet,
            )
            .await
            {
                Ok(Some(txhash)) => {
                    info!("[Pool] 0 gas Tx submitted, tx: {}", txhash);
                    self.zero_gas_nonce += 1;
                    return;
                }
                Ok(None) => {
                    info!("[Pool] 0 gas Tx failed, nonce: {}", self.zero_gas_nonce);
                    if reset {
                        let old_nonce = self.zero_gas_nonce;
                        self.reset_nonce().await;
                        if old_nonce != self.zero_gas_nonce {
                            return self.send(func, false).await;
                        }
                    }
                }
                Err(e) => {
                    error!("{}", e);
                }
            }
        }

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
