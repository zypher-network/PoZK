use clap::Args;
use api::Config;
use anyhow::Result;

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