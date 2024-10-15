use anyhow::{anyhow, Result};
use ethers::{prelude::*, types::transaction::eip2718::TypedTransaction};
use serde_json::{json, Value};
use std::sync::Arc;

// Vesting contract with abi
abigen!(Token, "../public/ABI/Token.json");

// Vesting contract with abi
abigen!(Vesting, "../public/ABI/Vesting.json");

// Epoch contract with abi
abigen!(Epoch, "../public/ABI/Epoch.json");

// Prover contract with abi
abigen!(Prover, "../public/ABI/Prover.json");

// Stake contract with abi
abigen!(Stake, "../public/ABI/Stake.json");

// Task contract with abi
abigen!(Task, "../public/ABI/Task.json");

// Reward contract with abi
abigen!(Reward, "../public/ABI/Reward.json");

// Controller contract with abi
abigen!(Controller, "../public/ABI/Controller.json");

const NETWORKS_ADDRESS: &str = include_str!("../../../public/networks.json");

pub fn contract_address(network: &str, name: &str) -> Result<(Address, u64)> {
    let addresses: Value =
        serde_json::from_str(NETWORKS_ADDRESS).map_err(|_| anyhow!("networks.json is invalid"))?;

    let address: Address = addresses[network][name]["address"]
        .as_str()
        .ok_or(anyhow!("contract address is invalid 1"))?
        .parse()
        .map_err(|_| anyhow!("contract address is invalid 2"))?;

    let start: i64 = addresses[network][name]["startBlock"]
        .as_i64()
        .ok_or(anyhow!("contract block is invalid"))?;

    Ok((address, start as u64))
}

pub fn pozk_metrics_url(network: &str) -> Result<String> {
    match network {
        "localhost" | "testnet" => Ok("https://pozk-metrics.zypher.dev".to_owned()),
        "mainnet" => Ok("https://pozk-metrics.zypher.network".to_owned()),
        _ => Err(anyhow!("Invalid network")),
    }
}

pub type DefaultProvider = Provider<Http>;

pub fn new_providers(rpcs: &[String]) -> Vec<Arc<DefaultProvider>> {
    let mut providers = vec![];
    for rpc in rpcs {
        if let Ok(p) = Provider::<Http>::try_from(rpc) {
            providers.push(Arc::new(p))
        }
    }
    providers
}

pub type DefaultSigner = SignerMiddleware<Arc<Provider<Http>>, LocalWallet>;

pub async fn new_signer(
    provider: Arc<DefaultProvider>,
    wallet: LocalWallet,
) -> Result<Arc<DefaultSigner>> {
    let signer = SignerMiddleware::new_with_provider_chain(provider, wallet).await?;
    Ok(Arc::new(signer))
}

pub async fn zero_gas<S: Signer>(uri: &str, tx: TypedTransaction, wallet: &S) -> Result<()> {
    let owner = wallet.address();
    let wallet = tx.from().unwrap();
    let to = tx.to_addr().unwrap();
    let data = tx.data().unwrap();
    let value = tx.value().unwrap();

    // TODO signature
    let v = "";
    let r = "";
    let s = "";

    let client = reqwest::Client::new();
    let data = json!({
        "wallet": format!("{:?}", wallet),
        "to": format!("{:?}", to),
        "data": format!("0x{}", hex::encode(data.to_vec())),
        "value": value.to_string(),
        "v": v,
        "r": r,
        "s": s,
        "owner": format!("{:?}", owner),
    });
    let _ = client.post(uri).json(&data).send().await?;

    todo!()
}
