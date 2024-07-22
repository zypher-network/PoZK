use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct Config {
    #[clap(long)]
    pub task_market_address: String,
    #[clap(long)]
    pub from: u64,
    #[clap(long)]
    pub delay_sec: u64,
    #[clap(long)]
    pub latest_step: u64,
    #[clap(long)]
    pub wait_time: u64,
    #[clap(long)]
    pub rpc_url: String,
    #[clap(long)]
    pub block_number: String,

    #[clap(long)]
    pub miner: String,
}
