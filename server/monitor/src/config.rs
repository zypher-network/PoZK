use anyhow::Result;
use clap::Args;
use ethers::prelude::Address;
use pozk_utils::contract_address;
use serde::Deserialize;

#[derive(Args, Debug, Clone, Deserialize)]
pub struct MonitorConfig {
    #[clap(long, help = "`monitor`: network type, localhost|testnet|mainnet")]
    pub network: String,

    #[clap(
        long,
        help = "`monitor`: RPC endpoints, e.g. https://example.com or https://example.com;https://example2.com"
    )]
    pub endpoints: String,

    #[clap(
        long,
        help = "`monitor`: miner account, e.g. 0x6cF0DE16160A1eF873f196aC9FB671e20598e2F8"
    )]
    pub miner: String,

    #[clap(
        long,
        help = "`monitor`: delay for network rollback, e.g. 1",
        default_value = "0"
    )]
    pub delay: u64,

    #[clap(
        long,
        help = "`monitor`: how many blocks to pull each time, e.g. 100",
        default_value = "100"
    )]
    pub step: u64,

    #[clap(long, help = "`monitor`: add 0 gas service", default_value = "")]
    pub zero_gas: String,

    #[clap(
        long,
        help = "`monitor`: monitor start height (Optional), e.g. 34736669"
    )]
    pub from: Option<u64>,

    #[clap(long, help = "`monitor`: special task contract (Optional)")]
    pub task_address: Option<String>,

    #[clap(long, help = "`monitor`: special prover contract (Optional)")]
    pub prover_address: Option<String>,

    #[clap(long, help = "`monitor`: special stake contract (Optional)")]
    pub stake_address: Option<String>,

    #[clap(long, help = "`monitor`: special controller contract (Optional)")]
    pub controller_address: Option<String>,
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            network: "mainnet".to_owned(),
            endpoints: "".to_owned(),
            miner: String::new(),
            delay: 0,
            step: 200,
            from: None,
            task_address: None,
            prover_address: None,
            stake_address: None,
            controller_address: None,
            zero_gas: "".to_owned(),
        }
    }
}

impl MonitorConfig {
    pub fn endpoints(&self) -> Vec<String> {
        self.endpoints.split(";").map(|x| x.to_owned()).collect()
    }

    pub fn miner(&self) -> Result<Address> {
        let miner: Address = self.miner.parse()?;
        Ok(miner)
    }

    pub fn task_address(&self) -> Result<(Address, Option<u64>)> {
        if let Some(t) = &self.task_address {
            let a: Address = t.parse()?;

            Ok((a, self.from))
        } else {
            let (a, f) = contract_address(&self.network, "Task")?;

            if self.from.is_none() {
                Ok((a, Some(f)))
            } else {
                Ok((a, self.from))
            }
        }
    }

    pub fn stake_address(&self) -> Result<(Address, Option<u64>)> {
        if let Some(t) = &self.stake_address {
            let a: Address = t.parse()?;

            Ok((a, self.from))
        } else {
            let (a, f) = contract_address(&self.network, "Stake")?;

            if self.from.is_none() {
                Ok((a, Some(f)))
            } else {
                Ok((a, self.from))
            }
        }
    }

    pub fn prover_address(&self) -> Result<(Address, Option<u64>)> {
        if let Some(t) = &self.prover_address {
            let a: Address = t.parse()?;

            Ok((a, self.from))
        } else {
            let (a, f) = contract_address(&self.network, "Prover")?;

            if self.from.is_none() {
                Ok((a, Some(f)))
            } else {
                Ok((a, self.from))
            }
        }
    }

    pub fn controller_address(&self) -> Result<(Address, Option<u64>)> {
        if let Some(t) = &self.controller_address {
            let a: Address = t.parse()?;

            Ok((a, self.from))
        } else {
            let (a, f) = contract_address(&self.network, "Controller")?;

            if self.from.is_none() {
                Ok((a, Some(f)))
            } else {
                Ok((a, self.from))
            }
        }
    }
}
