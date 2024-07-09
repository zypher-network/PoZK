mod web_server;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

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

fn main() -> Result<()> {
    let _args = Args::parse();

    Ok(())
}
