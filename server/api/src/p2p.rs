use anyhow::Result;
use chrono::Utc;
use ethers::prelude::*;
use pozk_db::{Prover, ReDB};
use pozk_docker::DockerManager;
use pozk_utils::pozk_metrics_url;
use reqwest::Client;
use serde_json::{json, Value};
use std::sync::Arc;
use sysinfo::System;
use tokio::{
    select,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender, Sender},
    time::sleep,
};
use chamomile::prelude::{start, Config, ReceiveMessage, SendMessage, PeerId, Peer};
use axum::extract::ws::{WebSocket, Message};
use futures_util::stream::SplitSink;

type WsChannel = SplitSink<WebSocket, Message>;

pub enum P2pMessage {
    ChangeController(Vec<u8>),
    NewProver(u64, WsChannel),
    NewPlayer(u64, WsChannel),
}

enum PlayerChannel {
    P2p,
    Ws(WsChannel),
}

struct P2pTask {
    /// room is viewable
    viewable: bool,
    /// timeout (max time in seconds)
    timeout: u64,
    /// prover websocket sender
    prover: WsChannel,
    /// task players
    players: HashMap<PeerId, PlayerChannel>,
    /// task viewers
    viewers: HashMap<PeerId, PlayerChannel>,
}

pub struct P2pService {
    path: PathBuf,
    db: Arc<ReDB>,
    docker: Arc<DockerManager>,
    tasks: HashMap<u64, P2pTask>,
}

enum P2pFuture {
    Out(P2pMessage),
    P2p(ReceiveMessage)
}

impl P2pService {
    pub fn new(path: PathBuf, db: Arc<ReDB>, docker: Arc<DockerManager>) -> Self {
        Self {
            path,
            db,
            docker,
            tasks: HashMap::new()
        }
    }

    pub fn run(self) -> UnboundedSender<P2pMessage> {
        let (sender, receiver) = unbounded_channel();
        tokio::spawn(self.listen(receiver));
        sender
    }

    async fn listen(mut self, mut recv: UnboundedReceiver<MetricsMessage>) {
        let (send, mut p2p_recv) = match recv.recv().await {
            Some(P2pMessage::ChangeController(sk)) => {
                start(path, port, sk).await
            }
            _ => return;
        };

        loop {
            let res = select! {
                v = async { recv.recv().await.and_then(|v| v.ok()).map(P2pFuture::Out) } => v,
                v = async { p2p_recv.recv().await.and_then(|v| v.ok()).map(P2pFuture::P2p) } => v,
            };

            match res {
                Some(P2pFuture::Out(msg)) => match msg {
                    P2pMessage::ChangeController(sk) => {
                        // stop old p2p
                        let _ = send.send(SendMessage::Network(NetworkType::NetworkStop)).await;

                        // sleep
                        sleep(Duration::from_secs(P2P_RESTART_TIME)).await;

                        // start new p2p
                        let (new_send, new_p2p_recv) = start(self.path, port, sk).await;
                        send = new_send;
                        p2p_recv = new_p2p_recv;
                    }
                }
                Some(P2pFuture::P2p(msg)) => match msg {
                    ReceiveMessage::Connect() => {
                        //
                    }
                }
                None => break;
            }
        }
    }
}

async fn start(path: PathBuf, port: u16, sk: Vec<u8>) {
    let socket: SockerAddr = "";
    let key = Key::from_db_bytes(&sk);

    let mut config = Config::default(Peer::socket(socket));
    config.permission = false;
    config.only_stable_data = true;
    config.db_dir = path;

    let (peer_id, send, out_recv) = start_with_key(config, key).await.unwrap();
    debug!("Peer id: {:?}", peer_id);

    (send, out_recv)
}
