use clap::Args;
use serde::Deserialize;

#[derive(Args, Debug, Clone, Deserialize)]
pub struct ApiConfig {
    #[clap(long, help = "`api`: service port, eg. 9098")]
    pub http_port: u16,

    #[clap(long, help = "`api`: p2p port, eg. 6374")]
    pub p2p_port: u16,

    #[clap(long, help = "`api`: service login param, eg. localhost:4000")]
    pub domains: Option<String>,

    #[clap(long, help = "`api`: owner, eg. 0x00...0000")]
    pub miner: String,

    #[clap(long, help = "`api`: service secret, eg. randomthisissecret")]
    pub secret: Option<String>,
}

impl ApiConfig {
    pub fn domains(&self) -> Vec<String> {
        let mut ds = vec![];
        if let Some(d) = &self.domains {
            for s in d.split(";") {
                ds.push(s.to_owned());
            }
        }
        ds
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            http_port: 9098,
            p2p_port: 7364,
            domains: None,
            miner: String::new(),
            secret: None,
        }
    }
}
