use anyhow::Result;
use api::Config;
use clap::Args;

#[derive(Args)]
pub struct Cmd {
    #[command(flatten)]
    config: Config,
}

impl Cmd {
    pub async fn exec(&self) -> Result<()> {
        Ok(())
    }
}
