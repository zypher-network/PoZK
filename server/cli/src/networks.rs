use anyhow::{anyhow, Result};
use ethers::prelude::Address;
use serde_json::Value;

const NETWORKS_ADDRESS: &str = include_str!("../../../public/networks.json");

pub fn contract_address(network: &str, name: &str) -> Result<(Address, u64)> {
    let addresses: Value =
        serde_json::from_str(NETWORKS_ADDRESS).map_err(|_| anyhow!("networks.json is invalid"))?;

    let address: Address = addresses[network][name]["address"]
        .as_str()
        .ok_or(anyhow!("contract address is invalid"))?
        .parse()
        .map_err(|_| anyhow!("contract address is invalid"))?;

    let start: i64 = addresses[network][name]["startBlock"]
        .as_i64()
        .ok_or(anyhow!("contract address is invalid"))?;

    Ok((address, start as u64))
}
