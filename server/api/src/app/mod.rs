mod controllers;
mod extensions;
pub use extensions::error::{Error, Result};

use anyhow::anyhow;
use axum::{
    extract::{Extension, Json},
    http::Method,
    middleware::from_extractor,
    routing::{get, post, Router},
};
use ethers::prelude::{Address, Http, Provider};
use pozk_db::ReDB;
use pozk_docker::DockerManager;
use pozk_utils::{contract_address, DefaultProvider, ServiceMessage, Task};
use rand::{thread_rng, Rng};
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use tower_http::cors::{Any, CorsLayer};

use controllers::*;
use extensions::auth::Auth;
use extensions::error::fallback;

use crate::config::ApiConfig;
use crate::p2p::P2pMessage;

pub fn success() -> Json<Value> {
    Json(json!({ "code": 0 }))
}

pub type AppContext = Arc<App>;

pub struct App {
    miner: Address,
    port: u16,
    domains: Vec<String>,
    db: Arc<ReDB>,
    docker: Arc<DockerManager>,
    sender: UnboundedSender<ServiceMessage>,
    p2p_sender: UnboundedSender<P2pMessage>,
    secret: [u8; 32],
    task: Task<DefaultProvider>,
    url: String,
}

impl App {
    pub fn new(
        cfg: &ApiConfig,
        db: Arc<ReDB>,
        docker: Arc<DockerManager>,
        sender: UnboundedSender<ServiceMessage>,
        p2p_sender: UnboundedSender<P2pMessage>,
        network: &str,
        endpoints: String,
        url: String,
    ) -> anyhow::Result<Self> {
        let miner: Address = cfg.miner.parse()?;
        let port = cfg.http_port;
        let domains = cfg.domains();

        let secret = if let Some(sec) = &cfg.secret {
            let mut secret = [0u8; 32];
            let mut hasher = Sha256::new();
            hasher.update(sec);
            let res = hasher.finalize();
            secret.copy_from_slice(res.as_slice());
            secret
        } else {
            thread_rng().gen::<[u8; 32]>()
        };

        let endpoint = endpoints
            .split(";")
            .next()
            .ok_or(anyhow!("Invalid endpoints"))?;
        let provider = Provider::<Http>::try_from(endpoint)?;
        let (task_address, _) = contract_address(network, "Task")?;
        let task = Task::new(task_address, Arc::new(provider));

        Ok(Self {
            miner,
            port,
            domains,
            db,
            docker,
            sender,
            p2p_sender,
            secret,
            task,
            url,
        })
    }

    pub fn run(self) {
        tokio::spawn(async move {
            let addr = SocketAddr::from(([0, 0, 0, 0], self.port));

            // cors
            let cors = CorsLayer::new()
                .allow_methods([Method::GET, Method::POST, Method::DELETE])
                .allow_headers(Any)
                .allow_origin(Any);

            let app = Router::new()
                .route("/login", post(auth::login))
                .route("/health", get(auth::health))
                .route("/orders", post(task::create))
                .route("/orders/:id", post(task::track))
                .route("/connect/:id", get(connect::player))
                .nest(
                    "/inner",
                    Router::new()
                        .route("/tasks/:id", get(task::download).post(task::upload))
                        .route("/connect/:id", get(connect::prover)),
                )
                .nest(
                    "/api",
                    Router::new()
                        .route(
                            "/controllers",
                            get(controller::index).post(controller::create),
                        )
                        .route(
                            "/controllers/:address",
                            get(controller::show).post(controller::update),
                        )
                        .route("/provers", get(prover::index).post(prover::create))
                        .route("/provers/:prover", get(prover::show).delete(prover::delete))
                        .route_layer(from_extractor::<Auth>()),
                )
                .route("/", get(auth::webapp))
                .route("/*path", get(auth::webapp))
                .layer(Extension(Arc::new(self)))
                .layer(cors)
                .fallback(fallback);

            info!("* HTTP listening: {}", addr);
            let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
            axum::serve(listener, app).await.unwrap();
        });
    }
}

#[derive(Deserialize, Debug)]
pub struct Pagination {
    pub page_count: usize,
    pub page_size: usize,
}

impl Pagination {
    pub fn parse(&self) -> (usize, usize) {
        let page_count = if self.page_count < 1 {
            1
        } else {
            self.page_count
        };

        let page_size = if self.page_size < 1 {
            10
        } else if self.page_size > 100 {
            100
        } else {
            self.page_size
        };

        (page_count, page_size)
    }

    pub fn begin_and_take(&self) -> (usize, usize) {
        let (page_count, page_size) = self.parse();

        let begin = if page_count == 1 {
            0
        } else {
            page_count * page_size
        };

        (begin, page_size)
    }
}
