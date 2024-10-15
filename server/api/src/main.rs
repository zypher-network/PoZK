#[macro_use]
extern crate tracing;

mod app;
mod config;
mod metrics;
mod service;

use app::App;
use config::ApiConfig;
use metrics::{MetricsMessage, MetricsService};
use service::MainService;

use anyhow::Result;
use clap::{Args, Parser};
use ethers::prelude::*;
use pozk_db::{Controller, DbConfig, MainController, ReDB};
use pozk_docker::DockerManager;
use pozk_monitor::{MonitorConfig, Pool, Scan};
use pozk_utils::{init_path_and_server, new_service_channel};
use serde::Deserialize;
use std::{fs, path::PathBuf, sync::Arc};

// empty account: sk = 0, address = 0x7e5f4552091a69125d5dfcb7b8c2659029395bdf
const DEFAULT_WALLET: &str = "0000000000000000000000000000000000000000000000000000000000000001";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Command {
    /// Config file for advance features
    #[arg(short, long)]
    config: Option<String>,

    /// base path for pozk, e.g. /usr/pozk(default), /home/ubuntu/pozk
    #[arg(short, long, default_value = "/usr/pozk")]
    base_path: String,

    /// base server for pozk, e.g. http://pozk-miner:9098(default), http://localhost:9098
    #[arg(short, long, default_value = "http://pozk-miner:9098")]
    server: String,

    /// Miner account, e.g. 0x00000000000000000000000000000000000000
    #[arg(short, long)]
    miner: String,

    /// RPC endpoint to listen and submit tx with chain
    #[arg(short, long)]
    endpoints: String,

    /// Network type, includes: localhost | testnet | mainnet
    #[arg(short, long)]
    network: String,

    /// Download docker image proxy (Optional), e.g. docker.registry.cyou
    #[arg(short, long)]
    docker_proxy: Option<String>,

    /// Use 0 gas service to send tx (Optional).
    #[arg(short, long)]
    zero_gas: Option<String>,
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
    tracing_subscriber::fmt::init();

    let args = Command::parse();

    let mut co = if let Some(path) = args.config {
        let toml_str = fs::read_to_string(path)?;
        toml::from_str(&toml_str)?
    } else {
        Config::default()
    };

    // update contract address
    co.monitor_config.network = args.network.clone();
    co.monitor_config.endpoints = args.endpoints.clone();
    co.monitor_config.miner = args.miner.clone();
    co.monitor_config.zero_gas = args.zero_gas.clone();
    co.api_config.miner = args.miner.clone();

    // setup base path
    init_path_and_server(&args.base_path, &args.server);

    // setup database
    let db = {
        let db_path = PathBuf::from(&args.base_path);
        let db = ReDB::new(&db_path, co.db_config.auto_remove)?;
        Arc::new(db)
    };

    // setup docker
    let docker = {
        let dm = DockerManager::new(args.docker_proxy)?;
        Arc::new(dm)
    };

    // start metrics service
    let metrics_sender = MetricsService::new(
        &args.network,
        args.miner.clone(),
        db.clone(),
        docker.clone(),
    )?
    .run();

    // setup controller
    let (controller, ready) =
        if let Some(addr) = db.get::<MainController>(MainController::to_key())? {
            (
                LocalWallet::from(
                    db.get::<Controller>(Controller::to_key(&addr.controller))?
                        .unwrap()
                        .singing_key,
                ),
                true,
            )
        } else {
            (DEFAULT_WALLET.parse::<LocalWallet>()?, false)
        };

    if ready {
        metrics_sender
            .send(MetricsMessage::ChangeController(controller.clone()))
            .unwrap();
    }

    let (service_sender, service_receiver) = new_service_channel();

    // setup monitor
    let pool_sender = Pool::new(&co.monitor_config, controller).await?.run();
    Scan::new(co.monitor_config, service_sender.clone(), db.clone())
        .await?
        .run();

    // setup api
    App::new(&co.api_config, db.clone(), service_sender)?.run();

    // setup main service
    MainService::new(pool_sender, metrics_sender, service_receiver, db, docker).run();

    tokio::signal::ctrl_c().await.unwrap();

    Ok(())
}
