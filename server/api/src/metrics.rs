use anyhow::Result;
use chrono::Utc;
use ethers::prelude::*;
use pozk_db::{Prover, ReDB};
use pozk_docker::DockerManager;
use pozk_utils::pozk_metrics_url;
use reqwest::Client;
use serde_json::{json, Value};
use std::{sync::Arc, time::Duration};
use sysinfo::System;
use tokio::{
    select,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    time::interval,
};

pub const PROXY_VERSION: &str = env!("CARGO_PKG_VERSION");

pub enum MetricsMessage {
    ChangeController(LocalWallet),
}

pub struct MetricsService {
    miner: String,
    wallet: Option<LocalWallet>,
    db: Arc<ReDB>,
    docker: Arc<DockerManager>,
    os: String,
    gpu: String,
    cpu: u64,
    memory: String,
    client: Client,
    metrics: String,
    url: String,
}

enum InnerFuture {
    Message(MetricsMessage),
    Report,
    Miner,
}

impl MetricsService {
    pub fn new(
        network: &str,
        miner: String,
        db: Arc<ReDB>,
        docker: Arc<DockerManager>,
        url: String,
    ) -> Result<Self> {
        let os = System::long_os_version().unwrap_or("Unknow".to_owned());
        let gpu = get_gpus();

        let mut sys = System::new();
        sys.refresh_all();
        let cpu = sys.cpus().len() as u64;
        let memory = format!(
            "{} GB",
            (sys.total_memory() * 100 / 1073741824) as f32 / 100f32
        ); // GB

        let client = reqwest::Client::new();
        let metrics = pozk_metrics_url(network)?;

        Ok(Self {
            miner,
            db,
            docker,
            os,
            gpu,
            cpu,
            memory,
            client,
            metrics,
            url,
            wallet: None,
        })
    }

    pub fn run(self) -> (UnboundedSender<MetricsMessage>, usize) {
        let cpu = self.cpu as usize;
        let (sender, receiver) = unbounded_channel();
        tokio::spawn(self.listen(receiver));
        (sender, cpu)
    }

    async fn listen(mut self, mut recv: UnboundedReceiver<MetricsMessage>) {
        let mut report_interval = interval(Duration::from_secs(600)); // 10min
        let mut miner_interval = interval(Duration::from_secs(3600)); // 1h
        loop {
            let work = select! {
                w = async {
                    recv.recv().await.map(InnerFuture::Message)
                } => w,
                w = async {
                    report_interval.tick().await;
                    Some(InnerFuture::Report)
                } => w,
                w = async {
                    miner_interval.tick().await;
                    Some(InnerFuture::Miner)
                } => w,
            };

            match work {
                Some(InnerFuture::Message(msg)) => match msg {
                    MetricsMessage::ChangeController(wallet) => {
                        self.wallet = Some(wallet);
                        if let Err(e) = self.report_miner_info().await {
                            error!("Report miner error: {}", e);
                        }
                    }
                },
                Some(InnerFuture::Report) => {
                    if let Err(e) = self.report_miner_healthy().await {
                        error!("Report prover error: {}", e);
                    }
                }
                Some(InnerFuture::Miner) => {
                    if let Err(e) = self.report_miner_info().await {
                        error!("Report miner error: {}", e);
                    }
                }
                None => break,
            }
        }
    }

    pub async fn report_miner_info(&self) -> Result<()> {
        let timestamp = Utc::now().timestamp();
        let (controller, signature) = if let Some(wallet) = &self.wallet {
            let controller = format!("{:?}", wallet.address());

            let message = format!(
                "{}{}{}{}{}{}{}{}{}",
                self.miner,
                controller,
                PROXY_VERSION,
                self.os,
                self.gpu,
                self.cpu,
                self.memory,
                self.url,
                timestamp,
            );
            let signature = wallet.sign_message(message).await?.to_string();
            (controller, signature)
        } else {
            return Ok(());
        };

        let data = json!({
            "miner": self.miner,
            "controller": controller,
            "version": PROXY_VERSION,
            "os": self.os,
            "gpu": self.gpu,
            "cpu": self.cpu,
            "memory": self.memory,
            "url": self.url,
            "timestamp": timestamp,
            "signature": signature,
        });

        let _res = self
            .client
            .post(format!("{}/miners", self.metrics))
            .json(&data)
            .send()
            .await?;

        Ok(())
    }

    // report every 10min
    async fn report_miner_healthy(&self) -> Result<()> {
        if self.wallet.is_none() {
            return Ok(());
        }

        let (provers, message) = list_provers(&self.docker, &self.db).await?;
        if provers.is_empty() {
            return Ok(());
        }

        let signature = if let Some(wallet) = &self.wallet {
            wallet.sign_message(message).await?.to_string()
        } else {
            return Ok(());
        };

        let data = json!({
            "miner": self.miner,
            "provers": provers,
            "timestamp": Utc::now().timestamp(),
            "signature": signature,
        });

        let _ = self
            .client
            .post(format!("{}/provers", self.metrics))
            .json(&data)
            .send()
            .await?;

        Ok(())
    }
}

pub async fn list_provers(
    docker: &Arc<DockerManager>,
    db: &Arc<ReDB>,
) -> Result<(Vec<Value>, String)> {
    let images = docker.list().await?;

    let (mut data, total) = db.list::<Prover>(0, 10)?;
    if total > 10 {
        let page = (total as f32 / 10f32).ceil() as usize;
        for i in 1..page {
            let (data1, _) = db.list::<Prover>(i * 10, 10)?;
            data.extend(data1)
        }
    }
    if data.is_empty() {
        return Ok((vec![], "".to_owned()));
    }

    let mut provers = vec![];
    let mut sign_provers = vec![];
    for p in data {
        // check image
        if images.contains_key(&p.image) {
            let prover = format!("{:?}", p.prover);
            provers.push(json!({
                "prover": prover,
                "version": p.tag,
            }));
            sign_provers.push(prover);
        }
    }
    let message = sign_provers.join(" ");

    Ok((provers, message))
}

fn get_gpus() -> String {
    // let instance: gfx_backend_vulkan::Instance = back::Instance::create("hayabusa", 1).unwrap();
    // let adapters: Vec<Adapter<Backend>> = instance.enumerate_adapters();

    // let mut names: Vec<String> = Vec::new();

    // for adapter in adapters {
    //     names.push(adapter.info.name.to_string());
    // }

    // names.join("")
    "".to_owned()
}
