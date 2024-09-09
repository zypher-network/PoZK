use clap::Args;
use serde::Deserialize;

#[derive(Args, Debug, Clone, Deserialize)]
pub struct MonitorConfig {
    #[clap(long, help = "`monitor`: open or close monitor")]
    pub open: bool,
    #[clap(
        long,
        help = "`monitor`: task market contract address, eg. 0x6cF0DE16160A1eF873f196aC9FB671e20598e2F8"
    )]
    pub task_address: String,
    #[clap(
        long,
        help = "`monitor`: prover market contract address, eg. 0x6cF0DE16160A1eF873f196aC9FB671e20598e2F8"
    )]
    pub prover_address: String,
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

    #[clap(
        long,
        help = "`monitor`: Download docker image proxy, eg. docker.registry.cyou"
    )]
    pub docker_proxy: Option<String>,
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            open: true,
            task_address: String::new(),
            prover_address: String::new(),
            stake_address: String::new(),
            from: 0,
            delay_sec: 0,
            step: 100,
            wait_time: 10,
            block_number_type: "latest".to_owned(),
            miner: String::new(),
            docker_proxy: None,
        }
    }
}
