use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct Config {
    #[clap(long)]
    pub host: String,

    #[clap(long)]
    pub port: u32,

    #[clap(long)]
    pub chain_id: u64,

    pub endpoint: String,

    pub domain: String,
}
