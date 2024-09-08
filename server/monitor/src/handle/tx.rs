use crate::utils;
use crate::utils::FuncType;
use anyhow::Result;
use db::{ControllerKey, ReDB};
use ethers::addressbook::Address;
use ethers::middleware::{Middleware, NonceManagerMiddleware, SignerMiddleware};
use ethers::prelude::{Http, LocalWallet, Provider};
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::{TransactionReceipt, U256};
use std::sync::Arc;
use std::time::Duration;
use tokio::spawn;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

/// If you need to wait for the result synchronously, use the sync field
pub struct TxChanData {
    pub sync: Option<UnboundedSender<TransactionReceipt>>,
    pub tx: TypedTransaction,
    pub func_type: FuncType,
}

impl TxChanData {
    pub fn sync_new(
        tx: TypedTransaction,
        func_type: FuncType,
    ) -> (Self, UnboundedReceiver<TransactionReceipt>) {
        let (sender, receiver) = unbounded_channel();
        (
            Self {
                sync: Some(sender),
                tx,
                func_type,
            },
            receiver,
        )
    }
}

pub struct TxService {
    db: Arc<ReDB>,
    eth_cli: Provider<Http>,
    tx_receiver: UnboundedReceiver<TxChanData>,
    tx_sender: UnboundedSender<TxChanData>,
    miner: Address,
    chain_id: U256,
}

impl TxService {
    pub fn new(
        eth_cli: Provider<Http>,
        db: Arc<ReDB>,
        miner: Address,
        chain_id: U256,
    ) -> Result<Self> {
        let (tx_sender, tx_receiver) = unbounded_channel();

        Ok(Self {
            db,
            eth_cli,
            tx_receiver,
            tx_sender,
            miner,
            chain_id,
        })
    }

    pub fn sender(&self) -> UnboundedSender<TxChanData> {
        self.tx_sender.clone()
    }

    pub fn run(mut self) {
        spawn(async move {
            let miner_key = ControllerKey(self.miner);

            while let Some(tx_data) = self.tx_receiver.recv().await {
                let ft = tx_data.func_type.clone();

                let eth_cli = {
                    let result = self.db.controller_set_entry(&miner_key);
                    match result {
                        Ok((key, signing_key)) => {
                            let wallet = match utils::gen_wallet(signing_key, self.chain_id) {
                                Ok(v) => v,
                                Err(e) => {
                                    log::error!(
                                        "[tx_service] func_type: {ft:?}, gen wallet: {e:?}"
                                    );
                                    continue;
                                }
                            };

                            let eth_cli = match utils::gen_nonce_manager_client(
                                &self.eth_cli,
                                &wallet,
                                key.0,
                            )
                            .await
                            {
                                Ok(v) => v,
                                Err(e) => {
                                    log::error!(
                                        "[tx_service] func_type: {ft:?}, gen nonce manager cli: {e:?}"
                                    );
                                    continue;
                                }
                            };

                            eth_cli
                        }
                        Err(e) => {
                            panic!(
                                "[tx_service] func_type: {ft:?}, tx service get controller set: {:?}",
                                e
                            )
                        }
                    }
                };

                let receipt = match Self::send_tx(&eth_cli, tx_data.tx).await {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("[tx_service] func_type: {ft:?}, send tx err: {e:?}");
                        continue;
                    }
                };

                if let Some(receipt) = receipt {
                    let code = if let Some(code) = receipt.status {
                        log::info!(
                            "[tx_service] func_type: {ft:?}, send tx, receipt status: [{}], tx_hash: [{:?}]",
                            code.as_u64(), receipt.transaction_hash
                        );

                        code.as_u64()
                    } else {
                        log::warn!(
                            "[tx_service] func_type: {ft:?}, send tx, receipt status is nil, tx_hash: [{:?}]",
                            receipt.transaction_hash
                        );
                        0
                    };

                    if code == 1 {
                        if let Some(sender) = tx_data.sync {
                            if let Err(e) = sender.send(receipt) {
                                log::error!("[tx_service] func_type: {ft:?}, send tx err: {e:?}");
                                continue;
                            }
                        }
                    }
                }
            }
        });
    }

    pub async fn send_tx(
        eth_cli: &NonceManagerMiddleware<SignerMiddleware<Provider<Http>, LocalWallet>>,
        tx: TypedTransaction,
    ) -> Result<Option<TransactionReceipt>> {
        let pending = eth_cli.send_transaction(tx, None).await?;

        let pending = pending.retries(3);

        let pending = pending.interval(Duration::from_secs(5));

        let receipt = pending.await?;

        Ok(receipt)
    }
}
