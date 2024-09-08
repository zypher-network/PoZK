mod redb;
pub use redb::*;

use clap::Args;
use serde::Deserialize;

#[derive(Args, Debug, Clone, Deserialize)]
pub struct DbConfig {
    #[clap(long, help = "`db`: auto remove db or not")]
    pub auto_remove: bool,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self { auto_remove: false }
    }
}
