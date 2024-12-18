#[macro_use]
extern crate tracing;

mod app;
mod config;
mod metrics;
mod p2p;
mod service;

use app::App;
use config::ApiConfig;
use metrics::{MetricsMessage, MetricsService};
use p2p::{P2pMessage, P2pService};
use service::MainService;

use anyhow::Result;
use clap::{Args, Parser};
use ethers::prelude::*;
use pozk_db::{Controller, DbConfig, MainController, ReDB};
use pozk_docker::DockerManager;
use pozk_monitor::{MonitorConfig, Pool, Scan};
use pozk_utils::{init_path_and_server, new_service_channel, pozk_rpc_url, pozk_zero_gas_url};
use serde::Deserialize;
use std::{fs, path::PathBuf, sync::Arc, time::Duration};

// empty account: sk = 0, address = 0x7e5f4552091a69125d5dfcb7b8c2659029395bdf
const DEFAULT_WALLET: &str = "0000000000000000000000000000000000000000000000000000000000000001";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Command {
    /// Network type, includes: localhost | testnet | mainnet
    #[arg(short, long)]
    network: String,

    /// Miner account, e.g. 0x00000000000000000000000000000000000000
    #[arg(short, long)]
    miner: String,

    /// base path for pozk, e.g. /usr/pozk(default), /home/ubuntu/pozk
    #[arg(short, long, default_value = "/Users/huangmingwei/pozk")]
    base_path: String,

    /// base server for pozk, e.g. http://pozk-miner:9098(default), http://localhost:9098
    #[arg(short, long, default_value = "http://pozk-miner:9098")]
    server: String,

    /// miner service url, e.g. https://example.com
    #[arg(short, long, default_value = "")]
    url: String,

    /// RPC endpoint to listen and submit tx with chain
    #[arg(short, long)]
    endpoints: Option<String>,

    /// Download docker image proxy (Optional), e.g. docker.registry.cyou
    #[arg(short, long)]
    docker_proxy: Option<String>,

    /// Use 0 gas service to send tx (Optional).
    #[arg(short, long)]
    zero_gas: Option<String>,

    /// Parallel tasks one time (Optional).
    #[arg(short, long)]
    parallel: Option<usize>,

    /// Config file for advance features
    #[arg(short, long)]
    config: Option<String>,

    /// ZKVM proxy service urL, e.g. http://127.0.0.1:9099
    #[arg(short = 'k', long)]
    zkvm: Option<String>,
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
async fn main() {
    setup().await.unwrap();
    error!("=== Stopped the miner service ===");
}

async fn setup() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Command::parse();

    let mut co = if let Some(path) = args.config {
        let toml_str = fs::read_to_string(path)?;
        toml::from_str(&toml_str)?
    } else {
        Config::default()
    };

    // check params
    let endpoints = args.endpoints.unwrap_or(pozk_rpc_url(&args.network)?);
    let zero_gas = args.zero_gas.unwrap_or(pozk_zero_gas_url(&args.network)?);
    let zkvm = args.zkvm.map(|v| v.trim_end_matches("/").to_owned());

    // update contract address
    co.monitor_config.network = args.network.clone();
    co.monitor_config.endpoints = endpoints.clone();
    co.monitor_config.miner = args.miner.clone();
    co.monitor_config.zero_gas = zero_gas;
    co.api_config.miner = args.miner.clone();

    // setup base path
    init_path_and_server(&args.base_path, &args.server);
    let base_path = PathBuf::from(&args.base_path);

    // setup database
    let db = {
        let db = ReDB::new(&base_path, co.db_config.auto_remove)?;
        Arc::new(db)
    };

    // setup docker
    let docker = {
        let dm = DockerManager::new(args.docker_proxy)?;
        Arc::new(dm)
    };

    // start metrics service
    let (metrics_sender, cpu) = MetricsService::new(
        &args.network,
        args.miner.clone(),
        db.clone(),
        docker.clone(),
        args.url.clone(),
    )?
    .run();

    let p2p_sender = P2pService::new(base_path, co.api_config.p2p_port).run();

    // calc parallel number
    let n_cpu = if cpu < 4 { 1 } else { cpu / 4 };
    let parallel = if let Some(p_cpu) = args.parallel {
        if p_cpu > n_cpu {
            warn!(
                "Parallel number is too large, maybe dangerous, recommend: {}",
                n_cpu
            );
        }
        p_cpu
    } else {
        n_cpu
    };

    // setup controller
    let (controller, sk_bytes, ready) =
        if let Some(addr) = db.get::<MainController>(MainController::to_key())? {
            let singing_key = db
                .get::<Controller>(Controller::to_key(&addr.controller))?
                .unwrap()
                .singing_key;
            let sk_bytes = singing_key.to_bytes().as_slice().to_vec();
            (LocalWallet::from(singing_key), sk_bytes, true)
        } else {
            (DEFAULT_WALLET.parse::<LocalWallet>()?, vec![], false)
        };

    if ready {
        // here use a sleep time to waiting api started.
        let init_controller = controller.clone();
        let init_metrics_sender = metrics_sender.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(10)).await;
            init_metrics_sender
                .send(MetricsMessage::ChangeController(init_controller))
                .unwrap();
        });

        p2p_sender
            .send(P2pMessage::ChangeController(sk_bytes))
            .unwrap();
    }

    let (service_sender, service_receiver) = new_service_channel();

    // setup monitor
    let pool_sender = Pool::new(&co.monitor_config, controller, ready)
        .await?
        .run();
    Scan::new(co.monitor_config, service_sender.clone(), db.clone())
        .await?
        .run();

    // setup api
    App::new(
        &co.api_config,
        db.clone(),
        docker.clone(),
        service_sender.clone(),
        p2p_sender.clone(),
        &args.network,
        endpoints,
        args.url.clone(),
        zkvm.clone(),
    )?
    .run();

    // setup main service
    MainService::new(
        pool_sender,
        metrics_sender,
        p2p_sender,
        service_receiver,
        db,
        docker,
        parallel,
        args.url,
        zkvm,
    )
    .run(service_sender);

    tokio::signal::ctrl_c().await?;

    Ok(())
}
