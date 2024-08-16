use anyhow::Result;
use api::{ApiConfig, ApiService};
use clap::{Args, Parser, Subcommand};
use db::ReDB;
use ethers::prelude::{Middleware, Provider, ProviderExt};
use ethers::types::Address;
use monitor::{init_functions, Monitor, MonitorConfig, ProverService, TaskService, TxService};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Parser)]
#[command(version, about, long_about)]
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
    let chain_id = eth_cli.get_chainid().await?;

    let db = {
        let db_path = PathBuf::from(&co.base_path);
        let db = ReDB::new(&db_path, co.db_remove)?;
        Arc::new(db)
    };

    let docker_manager = {
        let dm = docker::DockerManager::new()?;
        dm
    };

    if co.open_monitor {
        let task_market_address = Address::from_str(&co.monitor_config.task_market_address)?;
        let prover_market_address = Address::from_str(&co.monitor_config.prover_market_address)?;
        let stake_address = Address::from_str(&co.monitor_config.stake_address)?;

        let miner = Address::from_str(&co.monitor_config.miner)?;

        let mut monitor = Monitor::new(&co.monitor_config, eth_cli.clone()).await?;

        let tx_service = TxService::new(eth_cli.clone(), db.clone(), miner, chain_id)?;

        let task_service = TaskService::new(
            eth_cli.clone(),
            db.clone(),
            docker_manager.clone(),
            monitor.register(),
            tx_service.sender(),
            &co.base_path,
            chain_id,
            miner,
        )?;

        let prover_service = ProverService::new(
            eth_cli.clone(),
            db.clone(),
            docker_manager.clone(),
            monitor.register(),
            miner,
            co.monitor_config.docker_proxy_prefix,
            chain_id,
        )?;

        init_functions(task_market_address, stake_address, prover_market_address)?;

        tx_service.run();
        task_service.run();
        prover_service.run();

        monitor.run();
    }

    let _api = {
        let api = ApiService::new(&co.api_config, db, docker_manager, eth_cli).await?;

        api.run().await?;
    };

    tokio::signal::ctrl_c().await.unwrap();

    Ok(())
}
