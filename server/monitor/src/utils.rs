use crate::{PROVER_MARKET_CONTRACT_ABI, STAKE_CONTRACT_ABI, TASK_MARKET_CONTRACT_ABI};
use anyhow::{anyhow, Result};
use ethers::abi::{Contract, Function, Token};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::prelude::{
    Bytes, Http, LocalWallet, Middleware, NonceManagerMiddleware, Provider, Signer,
    SignerMiddleware, Wallet, U256, U64,
};
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::{Address, NameOrAddress, TransactionRequest};
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub async fn gen_nonce_manager_client(
    eth_cli: &Provider<Http>,
    wallet: &Wallet<SigningKey>,
    wallet_address: Address,
) -> Result<NonceManagerMiddleware<SignerMiddleware<Provider<Http>, LocalWallet>>> {
    let middleware = SignerMiddleware::new(eth_cli.clone(), wallet.clone());
    let client = NonceManagerMiddleware::new(middleware, wallet_address);
    Ok(client)
}

pub fn gen_wallet(signing_key: SigningKey, chain_id: U256) -> Result<Wallet<SigningKey>> {
    let mut wallet = LocalWallet::from(signing_key);
    wallet = wallet.with_chain_id(chain_id.as_u64());
    Ok(wallet)
}

pub async fn gen_tx(
    eth_cli: &Provider<Http>,
    data: Option<Bytes>,
    from: Option<Address>,
    to: Address,
    nonce: Option<U256>,
    chain_id: U256,
) -> Result<TypedTransaction> {
    let mut tx = TransactionRequest {
        from,
        to: Some(NameOrAddress::Address(to)),
        gas: None,
        gas_price: None,
        value: None,
        data,
        nonce,
        chain_id: Some(U64::from(chain_id.as_u64())),
    };

    if from.is_some() {
        let gas_price = eth_cli.get_gas_price().await?;
        tx.gas_price = Some(gas_price);

        let gas_limit = eth_cli
            .estimate_gas(&TypedTransaction::Legacy(tx.clone()), None)
            .await?;

        tx.gas = Some(gas_limit);
    }

    Ok(TypedTransaction::Legacy(tx))
}

pub fn gen_prover_name_data(prover: Address) -> Result<(Bytes, Function)> {
    let tx_data = PROVER_NAME_ABI.encode_input(&vec![Token::Address(prover)])?;

    Ok((Bytes::from(tx_data), PROVER_NAME_ABI.clone()))
}

pub fn gen_is_miner_data(prover: &Token, miner: Address) -> Result<(Address, Bytes, Function)> {
    let functions = FUNCTIONS.get().ok_or(anyhow!("functions not init"))?;

    if let Some((stake_address, func)) = functions.get(&FuncType::IsMiner) {
        let tx_data = func.encode_input(&vec![prover.clone(), Token::Address(miner)])?;

        Ok((stake_address.clone(), Bytes::from(tx_data), func.clone()))
    } else {
        Err(anyhow!("not match IsMiner func"))
    }
}

pub fn gen_prover_version_data(prover: &Token) -> Result<(Address, Bytes, Function)> {
    let functions = FUNCTIONS.get().ok_or(anyhow!("functions not init"))?;

    if let Some((prover_market_address, func)) = functions.get(&FuncType::ProverVersion) {
        let tx_data = func.encode_input(&vec![prover.clone()])?;
        Ok((
            prover_market_address.clone(),
            Bytes::from(tx_data),
            func.clone(),
        ))
    } else {
        Err(anyhow!("not match ProverVersion func"))
    }
}

pub fn gen_accept_task_data(id: Token, miner: Address) -> Result<(Address, Bytes, Function)> {
    let functions = FUNCTIONS.get().ok_or(anyhow!("functions not init"))?;

    if let Some((task_market_address, func)) = functions.get(&FuncType::AcceptTask) {
        let tx_data = func.encode_input(&vec![id, Token::Address(miner)])?;
        Ok((
            task_market_address.clone(),
            Bytes::from(tx_data),
            func.clone(),
        ))
    } else {
        Err(anyhow!("not match ProverVersion func"))
    }
}

pub fn gen_submit_data(id: Token, publics: Token, proof: Token) -> Result<(Address, Bytes)> {
    let functions = FUNCTIONS.get().ok_or(anyhow!("functions not init"))?;

    if let Some((task_market_address, func)) = functions.get(&FuncType::Submit) {
        let tx_data = func.encode_input(&vec![id, publics, proof])?;
        Ok((task_market_address.clone(), Bytes::from(tx_data)))
    } else {
        Err(anyhow!("not match ProverVersion func"))
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum FuncType {
    AcceptTask,
    IsMiner,
    ProverVersion,
    Submit,
}

static FUNCTIONS: OnceCell<BTreeMap<FuncType, (Address, Function)>> = OnceCell::new();

pub fn init_functions(
    task_market_address: Address,
    stake_address: Address,
    prover_market_address: Address,
) -> Result<()> {
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

    let _prover_market = {
        let prover_market = serde_json::from_str::<Contract>(PROVER_MARKET_CONTRACT_ABI)?;
        let version_func = prover_market
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

    FUNCTIONS
        .set(functions)
        .map_err(|e| anyhow!("init functions err: {e:?}"))?;

    Ok(())
}

static PROVER_NAME_ABI: Lazy<Function> = Lazy::new(|| {
    let json = r#"
			[
				{
					"type": "function",
					"name": "name",
					"inputs": [],
					"outputs": [
						{
						    "name":"",
							"type":"string",
							"internalType": "string"
						}
					]
				}
			]
		"#;
    let deserialized: Contract = serde_json::from_str(json).unwrap();
    deserialized.functions.get("name").unwrap()[0].clone()
});
