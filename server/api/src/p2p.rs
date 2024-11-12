use anyhow::Result;
use axum::extract::ws::{Message, WebSocket};
use chamomile::prelude::{start_with_key, Config, Key, Peer, PeerId, ReceiveMessage, SendMessage};
use chrono::Utc;
use ethers::prelude::*;
use futures_util::stream::SplitSink;
use pozk_db::{Prover, ReDB};
use pozk_docker::DockerManager;
use pozk_utils::pozk_metrics_url;
use reqwest::Client;
use serde_json::{json, Value};
use std::{collections::HashMap, net::SocketAddr, path::PathBuf, sync::Arc, time::Duration};
use sysinfo::System;
use tokio::{
    select,
    sync::mpsc::{unbounded_channel, Receiver, Sender, UnboundedReceiver, UnboundedSender},
    time::sleep,
};

type WsChannel = SplitSink<WebSocket, Message>;

const P2P_RESTART_TIME: u64 = 10;

pub enum P2pMessage {
    ChangeController(Vec<u8>),
    ConnectProver(u64, WsChannel),
    CloseProver(u64),
    ConnectPlayer(u64, PeerId, WsChannel),
    ClosePlayer(u64, PeerId),
    TextProver(u64, String),
    BinaryProver(u64, Vec<u8>),
    TextPlayer(u64, PeerId, String),
    BinaryPlayer(u64, PeerId, Vec<u8>),
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
    port: u16,
    db: Arc<ReDB>,
    docker: Arc<DockerManager>,
    tasks: HashMap<u64, P2pTask>,
}

enum P2pFuture {
    Out(P2pMessage),
    P2p(ReceiveMessage),
}

impl P2pService {
    pub fn new(path: PathBuf, port: u16, db: Arc<ReDB>, docker: Arc<DockerManager>) -> Self {
        Self {
            path,
            port,
            db,
            docker,
            tasks: HashMap::new(),
        }
    }

    pub fn run(self) -> UnboundedSender<P2pMessage> {
        let (sender, receiver) = unbounded_channel();
        tokio::spawn(self.listen(receiver));
        sender
    }

    async fn listen(mut self, mut recv: UnboundedReceiver<P2pMessage>) {
        let (mut send, mut p2p_recv) = match recv.recv().await {
            Some(P2pMessage::ChangeController(sk)) => {
                match start(self.path.clone(), self.port, sk).await {
                    Ok(res) => res,
                    Err(err) => {
                        error!("[p2p] start error (no service): {}", err);
                        return;
                    }
                }
            }
            _ => return,
        };

        loop {
            let res = select! {
                v = async { recv.recv().await.map(P2pFuture::Out) } => v,
                v = async { p2p_recv.recv().await.map(P2pFuture::P2p) } => v,
            };

            match res {
                Some(P2pFuture::Out(msg)) => match msg {
                    P2pMessage::ChangeController(sk) => {
                        // stop old p2p
                        let _ = send.send(SendMessage::NetworkStop).await;

                        // sleep
                        sleep(Duration::from_secs(P2P_RESTART_TIME)).await;

                        // start new p2p
                        let (new_send, new_p2p_recv) =
                            match start(self.path.clone(), self.port, sk).await {
                                Ok(res) => res,
                                Err(err) => {
                                    error!("[p2p] start error (use old): {}", err);
                                    continue;
                                }
                            };
                        send = new_send;
                        p2p_recv = new_p2p_recv;
                    }
                    P2pMessage::ConnectProver(tid, ws) => {
                        //
                    }
                    P2pMessage::CloseProver(tid) => {
                        //
                    }
                    P2pMessage::ConnectPlayer(tid, peer, ws) => {
                        //
                    }
                    P2pMessage::ClosePlayer(tid, peer) => {
                        //
                    }
                    P2pMessage::TextProver(tid, text) => {
                        //
                    }
                    P2pMessage::BinaryProver(tid, data) => {
                        //
                    }
                    P2pMessage::TextPlayer(tid, peer, text) => {
                        //
                    }
                    P2pMessage::BinaryPlayer(tid, peer, data) => {
                        //
                    }
                },
                Some(P2pFuture::P2p(msg)) => match msg {
                    ReceiveMessage::StableConnect(peer, data) => {
                        //
                    }
                    ReceiveMessage::StableLeave(peer) => {
                        //
                    }
                    ReceiveMessage::Data(peer, data) => {
                        //
                    }
                    _ => {}
                },
                None => break,
            }
        }
    }
}

async fn start(
    path: PathBuf,
    port: u16,
    sk: Vec<u8>,
) -> Result<(Sender<SendMessage>, Receiver<ReceiveMessage>)> {
    let socket = SocketAddr::from(([0, 0, 0, 0], port));
    let key = Key::from_db_bytes(&sk)?;

    let mut config = Config::default(Peer::socket(socket));
    config.permission = false;
    config.only_stable_data = true;
    config.db_dir = path;

    let (peer_id, send, out_recv) = start_with_key(config, key).await.unwrap();
    info!("[p2p] start network id: {:?} at {}", peer_id, port);

    Ok((send, out_recv))
}
