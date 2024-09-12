use anyhow::Result;
use api::{ApiConfig, ApiService};
use clap::{Args, Parser};
use db::{DbConfig, ReDB};
use ethers::prelude::{Middleware, Provider, ProviderExt};
use ethers::types::Address;
use monitor::{init_functions, Monitor, MonitorConfig, ProverService, TaskService, TxService};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

mod networks;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Command {
    /// Config file for advance features
    #[arg(short, long)]
    config: Option<String>,

    /// Host system path for pozk, e.g. /usr/pozk(default), /home/ubuntu/pozk
    #[arg(short = 'o', long)]
    host_base_path: Option<String>,

    /// Docker inner path, default is /usr/pozk
    #[arg(short, long)]
    docker_base_path: Option<String>,

    /// Miner account, e.g. 0x00000000000000000000000000000000000000
    #[arg(short, long)]
    miner: String,

    /// RPC endpoint to listen and submit tx with chain
    #[arg(short, long)]
    endpoint: String,

    /// Network type, includes: localhost | testnet | mainnet
    #[arg(short, long)]
    network: String,
}

#[derive(Args, Debug, Deserialize, Default)]
struct Config {
    #[clap(flatten)]
    db_config: DbConfig,

    #[clap(flatten)]
    api_config: ApiConfig,

    #[clap(flatten)]
    monitor_config: MonitorConfig,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = Command::parse();

    let miner = Address::from_str(&args.miner)?;

    // contract addresses
    let (stake_address, _s_start) = networks::contract_address(&args.network, "Stake")?;
    let (task_address, t_start) = networks::contract_address(&args.network, "Task")?;
    let (prover_address, _p_start) = networks::contract_address(&args.network, "Prover")?;

    let eth_cli = Provider::connect(&args.endpoint).await;
    let chain_id = eth_cli.get_chainid().await?;

    let mut co = if let Some(path) = args.config {
        let toml_str = fs::read_to_string(path)?;
        toml::from_str(&toml_str)?
    } else {
        let mut co = Config::default();
        co.monitor_config.from = t_start;
        co
    };

    // update contract address
    co.monitor_config.task_address = format!("{:?}", task_address);
    co.monitor_config.prover_address = format!("{:?}", prover_address);
    co.monitor_config.stake_address = format!("{:?}", stake_address);
    co.monitor_config.miner = args.miner.clone();
    co.api_config.miner = args.miner.clone();

    let host_base_path = if let Some(path) = args.host_base_path {
        path
    } else {
        "/usr/pozk".to_owned()
    };

    let docker_base_path = if let Some(path) = args.docker_base_path {
        path
    } else {
        "/usr/pozk".to_owned()
    };

    let db = {
        let db_path = PathBuf::from(&docker_base_path);
        let db = ReDB::new(&db_path, co.db_config.auto_remove)?;
        Arc::new(db)
    };

    let docker_manager = {
        let dm = docker::DockerManager::new()?;
        dm
    };

    if co.monitor_config.open {
        let mut monitor = Monitor::new(&co.monitor_config, eth_cli.clone()).await?;

        let tx_service = TxService::new(eth_cli.clone(), db.clone(), miner, chain_id)?;

        let task_service = TaskService::new(
            eth_cli.clone(),
            db.clone(),
            docker_manager.clone(),
            monitor.register(),
            tx_service.sender(),
            &docker_base_path,
            &host_base_path,
            chain_id,
            miner,
        )?;

        let prover_service = ProverService::new(
            eth_cli.clone(),
            db.clone(),
            docker_manager.clone(),
            monitor.register(),
            miner,
            co.monitor_config.docker_proxy,
            chain_id,
        )?;

        init_functions(task_address, stake_address, prover_address)?;

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
