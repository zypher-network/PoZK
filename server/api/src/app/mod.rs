mod controllers;
mod extensions;
pub use extensions::error::{Error, Result};

use axum::{
    extract::{Extension, Json},
    http::Method,
    middleware::from_extractor,
    routing::{get, post, Router},
};
use ethers::prelude::Address;
use pozk_db::ReDB;
use pozk_utils::ServiceMessage;
use rand::{thread_rng, Rng};
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use tower_http::cors::{Any, CorsLayer};

use controllers::auth;
use controllers::controller;
use controllers::prover;
use controllers::task;
use extensions::auth::Auth;
use extensions::error::fallback;

use crate::config::ApiConfig;

pub fn success() -> Json<Value> {
    Json(json!({ "status": "success" }))
}

pub type AppContext = Arc<App>;

pub struct App {
    miner: Address,
    port: u16,
    domains: Vec<String>,
    db: Arc<ReDB>,
    sender: UnboundedSender<ServiceMessage>,
    secret: [u8; 32],
}

impl App {
    pub fn new(
        cfg: &ApiConfig,
        db: Arc<ReDB>,
        sender: UnboundedSender<ServiceMessage>,
    ) -> anyhow::Result<Self> {
        let miner: Address = cfg.miner.parse()?;
        let port = cfg.port;
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

        Ok(Self {
            miner,
            port,
            domains,
            db,
            sender,
            secret,
        })
    }

    pub fn run(self) {
        tokio::spawn(async move {
            let addr = SocketAddr::from(([0, 0, 0, 0], self.port));

            // cors
            let cors = CorsLayer::new()
                .allow_methods([Method::GET, Method::POST, Method::DELETE])
                .allow_origin(Any);

            let app = Router::new()
                .route("/", get(auth::index))
                .route("/login", post(auth::login))
                .route("/tasks/:id", get(task::download).post(task::upload))
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
