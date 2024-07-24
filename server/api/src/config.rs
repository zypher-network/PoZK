use clap::Args;
use serde::Deserialize;

#[derive(Args, Debug, Clone, Deserialize)]
pub struct ApiConfig {
    #[clap(long, help = "`api`: service host, eg. 0.0.0.0")]
    pub host: String,

    #[clap(long, help = "`api`: service port, eg. 9098")]
    pub port: u32,

    #[clap(long, help = "`api`: service login param, eg. localhost:4000")]
    pub login_domain: String,
}
