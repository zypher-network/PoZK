use anyhow::{anyhow, Result};
use ethers::prelude::*;
use serde_json::Value;
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
