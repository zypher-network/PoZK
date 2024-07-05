mod web_server;

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(version, about, long_about, long_version = env!("BUILD_INFO_VERSION_LONG"))]
pub struct Args {
    #[clap(subcommand)]
    subcmd: Cmds,
}

#[derive(Subcommand)]
pub enum Cmds {
    WebServer,
}

fn main() -> Result<()>{
    let _args = Args::parse();

    Ok(())
}
