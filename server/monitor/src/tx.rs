use crate::event::EventType;
use crate::task::{TaskChanData, TaskType};
use crate::{PROVER_MARKET_CONTRACT_ABI, STAKE_CONTRACT_ABI, TASK_MARKET_CONTRACT_ABI};
use anyhow::{anyhow, Result};
use db::{ControllerKey, ReDB};
use docker::DockerManager;
use ethers::abi::{Bytes as AbiBytes, Contract, Function, Log as AbiLog, Token};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::nonce_manager::NonceManagerError;
use ethers::middleware::{NonceManagerMiddleware, SignerMiddleware};
use ethers::prelude::transaction::eip2718::TypedTransaction;
use ethers::prelude::{
    Http, LocalWallet, Middleware, PendingTransaction, Provider, ProviderError, ProviderExt,
    Signer, TransactionReceipt, U256,
};
use ethers::types::{Address, Bytes, NameOrAddress, TransactionRequest};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::format;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tokio::spawn;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

pub static REPO_PREFIX: &str = "zyphernetwork";

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TxChanData {
    pub ty: FuncType,
    pub data: BTreeMap<String, Token>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum FuncType {
    AcceptTask,
    IsMiner,
    ProverVersion,
    Submit,
}

impl From<&EventType> for FuncType {
    fn from(value: &EventType) -> Self {
        match value {
            EventType::CreateTask => FuncType::AcceptTask,
        }
    }
}

/// The transaction service currently only handles the CreateTask event,
/// so it is highly coupled and does not make any distinction.
/// If there are other events that need to be submitted after listening, we can make a distinction.
/// The main function of this object is to submit transactions,
/// so it is called a transaction service
pub struct TxService {
    db: Arc<ReDB>,
    eth_cli: Provider<Http>,
    tx_receiver: UnboundedReceiver<TxChanData>,
    task_sender: UnboundedSender<TaskChanData>,
    functions: BTreeMap<FuncType, (Address, Function)>,
    docker_manager: DockerManager,
    miner: Address,
}

impl TxService {
    pub fn new(
        db: Arc<ReDB>,
        tx_receiver: UnboundedReceiver<TxChanData>,
        task_sender: UnboundedSender<TaskChanData>,
        eth_cli: Provider<Http>,
        task_market_address: Address,
        stake_address: Address,
        prover_market_address: Address,
        docker_manager: DockerManager,
        miner: Address,
    ) -> Result<Self> {
        let mut functions = BTreeMap::new();

        let _task_market = {
            let task_market = serde_json::from_str::<Contract>(TASK_MARKET_CONTRACT_ABI)?;
            let accept_func = task_market
                .functions
                .get("accept")
                .ok_or(anyhow!("not match accept function"))?
                .get(0)
                .ok_or(anyhow!("functions[0] is nil"))?;
            functions.insert(
                FuncType::AcceptTask,
                (task_market_address, accept_func.clone()),
            );

            let submit_func = task_market
                .functions
                .get("submit")
                .ok_or(anyhow!("not match accept function"))?
                .get(0)
                .ok_or(anyhow!("functions[0] is nil"))?;
            functions.insert(FuncType::Submit, (task_market_address, submit_func.clone()));
        };

        let _stake = {
            let stake = serde_json::from_str::<Contract>(STAKE_CONTRACT_ABI)?;
            let is_miner_func = stake
                .functions
                .get("isMiner")
                .ok_or(anyhow!("not match isMiner function"))?
                .get(0)
                .ok_or(anyhow!("functions[0] is nil"))?;

            functions.insert(FuncType::IsMiner, (stake_address, is_miner_func.clone()));
        };

        let _game_market = {
            let game_market = serde_json::from_str::<Contract>(PROVER_MARKET_CONTRACT_ABI)?;
            let version_func = game_market
                .functions
                .get("version")
                .ok_or(anyhow!("not match version function"))?
                .get(0)
                .ok_or(anyhow!("functions[0] is nil"))?;
            functions.insert(
                FuncType::ProverVersion,
                (prover_market_address, version_func.clone()),
            );
        };

        Ok(Self {
            db,
            eth_cli,
            tx_receiver,
            task_sender,
            functions,
            docker_manager,
            miner,
        })
    }

    /// Procedure
    /// 1. get controller & wallet cli
    ///     The reason why it is necessary to obtain it every time is that if the user changes the controller from the front end,
    ///     the monitoring module cannot perceive it, so it will be obtained every time here.
    ///     Of course, a better approach is to make a thread dedicated to handling controller changes and monitor it,
    ///     and then set a global variable
    /// 2. get stake
    ///     Ensure users are eligible to mine
    /// 3. get docker image
    ///     Ensure that the user manually installs the image after staking
    /// 4. create tx_data & to
    /// 5. gen tx
    /// 6. send tx
    /// 7. wait receipt
    /// 8. judge receipt
    /// 9. send data to task chan
    pub fn run(mut self) {
        spawn(async move {
            let miner_key = ControllerKey(self.miner);

            while let Some(data) = self.tx_receiver.recv().await {
                // - get controller & wallet cli
                let (controller, eth_cli) = {
                    let result = self.db.controller_set_entry(&miner_key);
                    match result {
                        Ok((key, signing_key)) => {
                            let eth_cli = match self.gen_client(signing_key, key.0).await {
                                Ok(v) => v,
                                Err(e) => {
                                    log::error!("event: {data:?}, create wallet cli: {e:?}");
                                    continue;
                                }
                            };

                            (key.0, eth_cli)
                        }
                        Err(e) => {
                            panic!("event: {data:?}, tx service get controller set: {:?}", e)
                        }
                    }
                };

                match data.ty {
                    FuncType::AcceptTask => {
                        // - get stake
                        let prover = {
                            let func_type = FuncType::IsMiner;
                            let Some((stake_address, func)) = self.functions.get(&func_type) else {
                                log::warn!("event: {data:?}, not match IsMiner function");
                                continue;
                            };

                            let prover = {
                                let Some(prover) = data.data.get("prover") else {
                                    log::warn!("event: {data:?}, not match game");
                                    continue;
                                };
                                prover.clone()
                            };

                            let tx_data = match func
                                .encode_input(&vec![prover.clone(), Token::Address(self.miner)])
                            {
                                Ok(v) => v,
                                Err(e) => {
                                    log::error!(
                                        "event: {data:?}, func: {func_type:?}, encode input: {e:?}"
                                    );
                                    continue;
                                }
                            };

                            let tx = match self
                                .gen_tx(
                                    Some(Bytes::from(tx_data)),
                                    Some(controller),
                                    stake_address.clone(),
                                    None,
                                )
                                .await
                            {
                                Ok(v) => v,
                                Err(e) => {
                                    log::error!(
                                        "event: {data:?}, func: {func_type:?}, gen tx: {e:?}"
                                    );
                                    continue;
                                }
                            };

                            let res = match self
                                .eth_cli
                                .call(&TypedTransaction::Legacy(tx), None)
                                .await
                            {
                                Ok(v) => v,
                                Err(e) => {
                                    log::error!("event: {data:?}, IsMiner call: {e:?}");
                                    continue;
                                }
                            };

                            match func.decode_output(res.as_ref()) {
                                Ok(v) => {
                                    let Some(t) = v.get(0) else {
                                        log::warn!("event: {data:?}, IsMiner result decode list not index 0: {v:?}");
                                        continue;
                                    };

                                    let Some(is_miner) = t.clone().into_bool() else {
                                        log::warn!(
                                            "event: {data:?}, IsMiner result not match bool"
                                        );
                                        continue;
                                    };

                                    if !is_miner {
                                        log::warn!(
                                            "event: {data:?}, controller: {controller:?} not miner"
                                        );
                                        continue;
                                    }
                                }
                                Err(e) => {
                                    log::error!(
                                        "event: {data:?}, decode IsMiner call result: {e:?}"
                                    );
                                    continue;
                                }
                            };

                            prover
                        };

                        // - get docker image
                        let (repo, tag, prover_address) = {
                            let (repo, prover_address) = {
                                let Some(prover) = prover.clone().into_address() else {
                                    log::warn!("prover: {prover:?} not address type");
                                    continue;
                                };

                                (format!("{REPO_PREFIX}/{prover:?}"), prover)
                            };

                            let tag = {
                                let func_type = FuncType::ProverVersion;
                                let Some((game_market_address, func)) =
                                    self.functions.get(&func_type)
                                else {
                                    log::warn!("event: {data:?}, func type: {func_type:?} not match in self.functions");
                                    continue;
                                };

                                let prover = prover.clone();

                                let tx_data = match func.encode_input(&vec![prover.clone()]) {
                                    Ok(v) => v,
                                    Err(e) => {
                                        log::error!("event: {data:?}, func: {func_type:?}, encode input: {e:?}");
                                        continue;
                                    }
                                };

                                let tx = match self
                                    .gen_tx(
                                        Some(Bytes::from(tx_data)),
                                        Some(controller),
                                        game_market_address.clone(),
                                        None,
                                    )
                                    .await
                                {
                                    Ok(v) => v,
                                    Err(e) => {
                                        log::error!("func: {:?}, gen tx: {:?}", func_type, e);
                                        continue;
                                    }
                                };

                                let res = match self
                                    .eth_cli
                                    .call(&TypedTransaction::Legacy(tx), None)
                                    .await
                                {
                                    Ok(v) => v,
                                    Err(e) => {
                                        log::error!("IsMiner call: {e:?}");
                                        continue;
                                    }
                                };

                                let tag = match func.decode_output(res.as_ref()) {
                                    Ok(v) => {
                                        let Some(t) = v.get(0) else {
                                            log::warn!(
                                                "IsMiner result decode list not index 0: {v:?}"
                                            );
                                            continue;
                                        };

                                        let Some(version) = t.clone().into_uint() else {
                                            log::warn!("version result not match uint");
                                            continue;
                                        };

                                        version.to_string()
                                    }
                                    Err(e) => {
                                        log::error!("get game version : {prover:?} {e:?}");
                                        continue;
                                    }
                                };

                                tag
                            };

                            match self.docker_manager.image_exist(&repo, &tag).await {
                                Ok(v) => {
                                    if !v {
                                        log::warn!("repo: {repo}, tag: {tag} local not install");
                                        continue;
                                    }
                                }
                                Err(e) => {
                                    log::error!("query repo: {repo}, tag: {tag} err: {e:?}");
                                    continue;
                                }
                            }

                            (repo, tag, prover_address)
                        };

                        // - create tx_data & to & task id
                        let (tx_data, to, tid) = {
                            let func_type = FuncType::AcceptTask;
                            let Some((task_market_address, func)) = self.functions.get(&func_type)
                            else {
                                log::warn!("event: {data:?}, not match in self.functions");
                                continue;
                            };

                            let (id, tid) = {
                                let Some(id) = data.data.get("id") else {
                                    log::warn!("event: {data:?}, not match id");
                                    continue;
                                };

                                let Some(tid) = id.clone().into_uint() else {
                                    log::warn!("event: {data:?}, id to uint nil");
                                    continue;
                                };

                                (id.clone(), tid)
                            };

                            let tx_data =
                                match func.encode_input(&vec![id, Token::Address(self.miner)]) {
                                    Ok(v) => v,
                                    Err(e) => {
                                        log::error!(
                                        "event: {data:?}, func: {func_type:?}, encode input: {e:?}"
                                    );
                                        continue;
                                    }
                                };

                            (tx_data, task_market_address.clone(), tid)
                        };

                        let receipt = match self.send_tx(&eth_cli, tx_data, to, controller).await {
                            Ok(v) => v,
                            Err(e) => {
                                log::error!("event: {data:?}, send tx: {e:?}");
                                continue;
                            }
                        };

                        if let Some(receipt) = receipt {
                            // - judge receipt
                            let code = if let Some(code) = receipt.status {
                                log::info!(
                                    "event: {data:?}, send tx, receipt status: [{}]",
                                    code.as_u64()
                                );

                                code.as_u64()
                            } else {
                                log::warn!("event: {data:?}, send tx, receipt status is nil");
                                0
                            };

                            // - send task
                            if code == 1 {
                                let input = {
                                    let Some(input) = data.data.get("data") else {
                                        log::warn!("event: {data:?}, not match data");
                                        continue;
                                    };

                                    let Some(input) = input.clone().into_bytes() else {
                                        log::warn!("event: {data:?}, input to bytes nil");
                                        continue;
                                    };

                                    input
                                };

                                let sender = self.task_sender.clone();
                                let task_data = TaskChanData {
                                    ty: TaskType::RunTask,
                                    data: input,
                                    repo,
                                    prover: prover_address,
                                    tag,
                                    tid,
                                    miner: miner_key.clone(),
                                    controller: ControllerKey(controller),
                                };
                                match sender.send(task_data) {
                                    Ok(_) => {
                                        log::info!("event: {data:?}, send task success");
                                    }
                                    Err(e) => {
                                        log::error!("event: {data:?}, send task err: {e:?}");
                                    }
                                }
                            }
                        } else {
                            log::warn!("event: {data:?}, receipt is nil");
                        };
                    }
                    FuncType::Submit => {
                        // - create tx_data
                        let (tx_data, to) = {
                            let func_type = FuncType::Submit;
                            let Some((task_market_address, func)) = self.functions.get(&func_type)
                            else {
                                log::warn!("event: {data:?}, not match in self.functions");
                                continue;
                            };

                            let Some(id) = data.data.get("id") else {
                                log::warn!("event: {data:?}, not match id");
                                continue;
                            };

                            let Some(publics) = data.data.get("publics") else {
                                log::warn!("event: {data:?}, not match publics");
                                continue;
                            };

                            let Some(proof) = data.data.get("proof") else {
                                log::warn!("event: {data:?}, not match proof");
                                continue;
                            };

                            let tx_data = match func.encode_input(&vec![
                                id.clone(),
                                publics.clone(),
                                proof.clone(),
                            ]) {
                                Ok(v) => v,
                                Err(e) => {
                                    log::error!(
                                        "event: {data:?}, func: {func_type:?}, encode input: {e:?}"
                                    );
                                    continue;
                                }
                            };

                            (tx_data, task_market_address.clone())
                        };

                        let receipt = match self.send_tx(&eth_cli, tx_data, to, controller).await {
                            Ok(v) => v,
                            Err(e) => {
                                log::error!("event: {data:?}, send tx: {e:?}");
                                continue;
                            }
                        };

                        if let Some(receipt) = receipt {
                            if let Some(code) = receipt.status {
                                log::info!(
                                    "event: {data:?}, send tx, receipt status: [{}]",
                                    code.as_u64()
                                );
                            } else {
                                log::warn!("event: {data:?}, send tx, receipt status is nil");
                            }
                        } else {
                            log::warn!("event: {data:?}, receipt is nil");
                        };
                    }
                    _ => {}
                }
            }
        });
    }

    pub async fn send_tx(
        &self,
        eth_cli: &NonceManagerMiddleware<SignerMiddleware<Provider<Http>, LocalWallet>>,
        tx_data: AbiBytes,
        to: Address,
        controller: Address,
    ) -> Result<Option<TransactionReceipt>> {
        let tx = self
            .gen_tx(Some(Bytes::from(tx_data)), Some(controller), to, None)
            .await?;

        let pending = eth_cli
            .send_transaction(TypedTransaction::Legacy(tx), None)
            .await?;

        let pending = pending.retries(3);

        let pending = pending.interval(Duration::from_secs(5));

        let receipt = pending.await?;

        Ok(receipt)
    }

    pub async fn gen_tx(
        &self,
        data: Option<Bytes>,
        from: Option<Address>,
        to: Address,
        nonce: Option<U256>,
    ) -> Result<TransactionRequest> {
        let mut tx = TransactionRequest {
            from,
            to: Some(NameOrAddress::Address(to)),
            gas: None,
            gas_price: None,
            value: None,
            data,
            nonce,
            chain_id: None,
        };

        if from.is_some() {
            let gas_price = self.eth_cli.get_gas_price().await?;
            tx.gas_price = Some(gas_price);

            let gas_limit = self
                .eth_cli
                .estimate_gas(&TypedTransaction::Legacy(tx.clone()), None)
                .await?;
            tx.gas = Some(gas_limit);
        }

        log::debug!("tx: [{tx:?}]");

        Ok(tx)
    }

    pub async fn gen_client(
        &self,
        signing_key: SigningKey,
        address: Address,
    ) -> Result<NonceManagerMiddleware<SignerMiddleware<Provider<Http>, LocalWallet>>> {
        let chain_id = self.eth_cli.get_chainid().await?;
        let mut wallet = LocalWallet::from(signing_key);
        wallet = wallet.with_chain_id(chain_id.as_u64());

        let middleware = SignerMiddleware::new(self.eth_cli.clone(), wallet);
        let client = NonceManagerMiddleware::new(middleware, address);
        Ok(client)
    }
}

#[cfg(test)]
mod test {
    use crate::event::EventType;
    use crate::tx::{TxChanData, TxService};
    use db::ReDB;
    use docker::DockerManager;
    use ethers::prelude::{Provider, ProviderExt};
    use ethers::types::Address;
    use std::collections::BTreeMap;
    use std::path::PathBuf;
    use std::str::FromStr;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::spawn;
    use tokio::sync::mpsc::unbounded_channel;

    // #[test]
    // fn test_tx_service() {
    //     env_logger::init();
    //     let rt = tokio::runtime::Builder::new_current_thread()
    //         .enable_all()
    //         .build()
    //         .unwrap();
    //     rt.block_on(async {
    //         let db = {
    //             let db_path = PathBuf::from("/tmp/pozk");
    //             let db = ReDB::new(&db_path, true).unwrap();
    //             Arc::new(db)
    //         };
    //
    //         let (sender, receiver) = unbounded_channel();
    //
    //         let task_market_address = Address::from_str("0x").unwrap();
    //         let stake_address = Address::from_str("0x").unwrap();
    //         let game_market_address = Address::from_str("0x").unwrap();
    //
    //         let miner = Address::from_str("0x").unwrap();
    //
    //         let docker_manager = DockerManager::new().unwrap();
    //
    //         let opbnb_testnet_cli = Provider::connect("http://127.0.0.1:8545").await;
    //
    //         let tx_service = TxService::new(
    //             db,
    //             receiver,
    //             opbnb_testnet_cli,
    //             task_market_address,
    //             stake_address,
    //             game_market_address,
    //             docker_manager,
    //             miner,
    //         )
    //         .unwrap();
    //
    //         tx_service.run();
    //
    //         tokio::time::sleep(Duration::from_secs(5)).await;
    //
    //         sender
    //             .send(TxChanData {
    //                 ty: EventType::CreateTask,
    //                 data: BTreeMap::new(),
    //             })
    //             .unwrap();
    //
    //         tokio::signal::ctrl_c().await.unwrap();
    //     });
    // }
}
