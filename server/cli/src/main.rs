use anyhow::Result;
use api::{ApiConfig, ApiService};
use clap::{Args, Parser, Subcommand};
use db::ReDB;
use ethers::prelude::{Provider, ProviderExt};
use ethers::types::Address;
use monitor::{Monitor, MonitorConfig, TaskService, TxService};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::mpsc::unbounded_channel;

#[derive(Parser)]
#[command(version, about, long_about, long_version = env!("BUILD_INFO_VERSION_LONG"))]
pub struct Command {
    #[command(subcommand)]
    pub sub: SubCmd,
}
#[derive(Subcommand, Debug)]
pub enum SubCmd {
    #[command(about = "Use a configuration file")]
    File(ConfigFile),

    #[command(about = "Use configuration options")]
    Option(ConfigOption),
}

#[derive(Args, Debug)]
pub struct ConfigFile {
    #[clap(long, help = "Toml configuration file path")]
    pub path: String,
}

#[derive(Args, Debug, Deserialize)]
pub struct ConfigOption {
    #[clap(long, help = "redb and prover file path, eg. /tmp/pozk/")]
    pub base_path: String,

    #[clap(long, help = "Whether to delete db")]
    pub db_remove: bool,

    #[clap(long, help = "blockchain rpc, eg. http://127.0.0.1:8545")]
    pub endpoint: String,

    #[clap(long, help = "Whether to delete db")]
    pub open_monitor: bool,

    #[clap(long, help = "docker update url, eg. http://127.0.0.1:7777/XXX")]
    pub docker_update_url: Option<String>,

    #[clap(flatten)]
    pub api_config: ApiConfig,

    #[clap(flatten)]
    pub monitor_config: MonitorConfig,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = Command::parse();

    let co = match args.sub {
        SubCmd::File(cf) => {
            let toml_str = fs::read_to_string(cf.path)?;
            let co: ConfigOption = toml::from_str(&toml_str)?;
            co
        }
        SubCmd::Option(co) => co,
    };

    let eth_cli = Provider::connect(&co.endpoint).await;

    let db = {
        let db_path = PathBuf::from(&co.base_path);
        let db = ReDB::new(&db_path, true)?;
        Arc::new(db)
    };

    let docker_manager = {
        let dm = docker::DockerManager::new()?;
        dm
    };

    if co.open_monitor {
        let (tx_sender, tx_receiver) = unbounded_channel();
        let (task_sender, task_receiver) = unbounded_channel();

        let task_market_address = Address::from_str(&co.monitor_config.task_market_address)?;
        let prover_market_address = Address::from_str(&co.monitor_config.prover_market_address)?;
        let stake_address = Address::from_str(&co.monitor_config.stake_address)?;
        let miner = Address::from_str(&co.monitor_config.miner)?;

        // create monitor
        let monitor = Monitor::new(&co.monitor_config, eth_cli.clone(), tx_sender.clone()).await?;

        // create tx service
        let tx_service = TxService::new(
            db.clone(),
            tx_receiver,
            task_sender,
            eth_cli.clone(),
            task_market_address,
            stake_address,
            prover_market_address,
            docker_manager.clone(),
            miner,
        )?;

        // create task service
        let task_service = TaskService::new(
            db.clone(),
            docker_manager.clone(),
            task_receiver,
            tx_sender.clone(),
            &co.base_path,
        )?;

        monitor.run();
        tx_service.run();
        task_service.run();
    }

    let _api = {
        let api = ApiService::new(&co.api_config, db, docker_manager, eth_cli).await?;

        api.run().await?;
    };

    tokio::signal::ctrl_c().await.unwrap();

    Ok(())
}
