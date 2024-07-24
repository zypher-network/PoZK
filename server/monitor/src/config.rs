use clap::Args;
use serde::Deserialize;

#[derive(Args, Debug, Clone, Deserialize)]
pub struct MonitorConfig {
    #[clap(
        long,
        help = "`monitor`: task market contract address, eg. 0x6cF0DE16160A1eF873f196aC9FB671e20598e2F8"
    )]
    pub task_market_address: String,
    #[clap(
        long,
        help = "`monitor`: prover market contract address, eg. 0x6cF0DE16160A1eF873f196aC9FB671e20598e2F8"
    )]
    pub prover_market_address: String,
    #[clap(
        long,
        help = "`monitor`: stake contract address, eg. 0x6cF0DE16160A1eF873f196aC9FB671e20598e2F8"
    )]
    pub stake_address: String,
    #[clap(long, help = "`monitor`: monitor start height, eg. 34736669")]
    pub from: u64,
    #[clap(
        long,
        help = "`monitor`: solve the configuration of rollback and delay the acquisition of events, eg. 60"
    )]
    pub delay_sec: u64,
    #[clap(long, help = "`monitor`: how many blocks to pull each time, eg. 10")]
    pub step: u64,
    #[clap(
        long,
        help = "`monitor`: if the number of blocks pulled is insufficient, the waiting time will be executed, eg. 10"
    )]
    pub wait_time: u64,
    #[clap(
        long,
        help = "`monitor`: get the type of block, there are `Latest`, `Finalized`, `Safe`, the default is `Latest`, eg. Latest"
    )]
    pub block_number_type: String,
    #[clap(
        long,
        help = "`monitor`: owner, eg. 0x6cF0DE16160A1eF873f196aC9FB671e20598e2F8"
    )]
    pub miner: String,
}
