use anyhow::Result;
use axum::extract::ws::{Message, WebSocket};
use chamomile::prelude::{start_with_key, Config, Key, Peer, PeerId, ReceiveMessage, SendMessage};
use chrono::Utc;
use ethers::prelude::Address;
use futures_util::{stream::SplitSink, SinkExt};
use pozk_utils::{BinaryMessage, TextMessage};
use std::{collections::HashMap, net::SocketAddr, path::PathBuf, time::Duration};
use tokio::{
    select,
    sync::mpsc::{unbounded_channel, Receiver, Sender, UnboundedReceiver, UnboundedSender},
    time::sleep,
};

type WsChannel = SplitSink<WebSocket, Message>;

const P2P_RESTART_TIME: u64 = 10;
const P2P_CLEAR_TIME: u64 = 3600; // 1h

pub enum P2pMessage {
    ChangeController(Vec<u8>),
    ConnectProver(u64, WsChannel, bool, bool, i64),
    CloseProver(u64),
    ConnectPlayer(u64, Address, String, WsChannel),
    ClosePlayer(u64, Address),
    TextProver(u64, String),
    BinaryProver(u64, Vec<u8>),
    TextPlayer(u64, Address, String),
    BinaryPlayer(u64, Address, Vec<u8>),
}

enum PlayerChannel {
    Ws(WsChannel),
    P2p(PeerId),
    None,
}

impl PlayerChannel {
    async fn close(&mut self, p2p_sender: &Sender<SendMessage>) {
        match self {
            PlayerChannel::Ws(ws) => {
                let _ = ws.send(Message::Close(None)).await;
            }
            PlayerChannel::P2p(peer) => {
                let _ = p2p_sender.send(SendMessage::StableDisconnect(*peer)).await;
            }
            PlayerChannel::None => {}
        }
    }

    async fn text(&mut self, p2p_sender: &Sender<SendMessage>, msg: String) {
        match self {
            PlayerChannel::Ws(ws) => {
                let _ = ws.send(Message::Text(msg)).await;
            }
            PlayerChannel::P2p(peer) => {
                let _ = p2p_sender
                    .send(SendMessage::Data(0, *peer, msg.into_bytes()))
                    .await;
            }
            PlayerChannel::None => {}
        }
    }

    async fn binary(&mut self, p2p_sender: &Sender<SendMessage>, data: Vec<u8>) {
        match self {
            PlayerChannel::Ws(ws) => {
                let _ = ws.send(Message::Binary(data)).await;
            }
            PlayerChannel::P2p(peer) => {
                let _ = p2p_sender.send(SendMessage::Data(0, *peer, data)).await;
            }
            PlayerChannel::None => {}
        }
    }
}

struct P2pTask {
    /// room is viewable
    viewable: bool,
    /// game started, after started, next connect player will be viewer
    started: bool,
    /// overtime at (timestamp)
    overtime: i64,
    /// prover websocket sender
    prover: WsChannel,
    /// task players
    players: HashMap<Address, PlayerChannel>,
    /// task viewers
    viewers: HashMap<Address, PlayerChannel>,
}

pub struct P2pService {
    path: PathBuf,
    port: u16,
    tasks: HashMap<u64, P2pTask>,
    peers: HashMap<Address, u64>,
}

enum P2pFuture {
    Out(P2pMessage),
    P2p(ReceiveMessage),
    Clear,
}

impl P2pService {
    pub fn new(path: PathBuf, port: u16) -> Self {
        Self {
            path,
            port,
            tasks: HashMap::new(),
            peers: HashMap::new(),
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
                v = async {
                    sleep(Duration::from_secs(P2P_CLEAR_TIME)).await;
                    Some(P2pFuture::Clear)
                } => v,
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
                    P2pMessage::ConnectProver(tid, ws, viewable, started, overtime) => {
                        if let Some(task) = self.tasks.get_mut(&tid) {
                            task.prover = ws;
                        } else {
                            self.tasks.insert(
                                tid,
                                P2pTask {
                                    viewable,
                                    started,
                                    overtime,
                                    prover: ws,
                                    players: HashMap::new(),
                                    viewers: HashMap::new(),
                                },
                            );
                        }
                    }
                    P2pMessage::CloseProver(tid) => {
                        let mut remove = false;
                        if let Some(task) = self.tasks.get_mut(&tid) {
                            let now = Utc::now().timestamp();
                            if task.overtime < now {
                                remove = true;
                                // clear players & viewers
                                for (_, mut p) in task.players.drain() {
                                    p.close(&send).await;
                                }
                                for (_, mut p) in task.viewers.drain() {
                                    p.close(&send).await;
                                }
                            }
                        }
                        if remove {
                            let _ = self.tasks.remove(&tid);
                        }
                    }
                    P2pMessage::ConnectPlayer(tid, peer, data, mut ws) => {
                        if let Some(task) = self.tasks.get_mut(&tid) {
                            if !task.started || task.players.contains_key(&peer) {
                                task.players.insert(peer, PlayerChannel::Ws(ws));
                                let text = TextMessage::ConnectPlayer(peer, data).encode();
                                let _ = task.prover.send(Message::Text(text)).await;
                            } else if task.viewable {
                                task.viewers.insert(peer, PlayerChannel::Ws(ws));
                                let text = TextMessage::ConnectViewer(peer).encode();
                                let _ = task.prover.send(Message::Text(text)).await;
                            } else {
                                let _ = ws.send(Message::Close(None)).await;
                            }
                        } else {
                            let _ = ws.send(Message::Close(None)).await;
                        }
                    }
                    P2pMessage::ClosePlayer(tid, peer) => {
                        if let Some(task) = self.tasks.get_mut(&tid) {
                            let text = if task.players.contains_key(&peer) {
                                if task.started {
                                    task.players.insert(peer, PlayerChannel::None);
                                } else {
                                    let _ = task.players.remove(&peer);
                                }

                                TextMessage::ClosePlayer(peer)
                            } else {
                                let _ = task.viewers.remove(&peer);
                                TextMessage::CloseViewer(peer)
                            };
                            let _ = task.prover.send(Message::Text(text.encode())).await;
                        }
                    }
                    P2pMessage::TextProver(tid, text) => {
                        // handler inner function
                        match TextMessage::decode(text) {
                            TextMessage::Started => {
                                if let Some(task) = self.tasks.get_mut(&tid) {
                                    task.started = true;
                                }
                            }
                            TextMessage::Over => {
                                if let Some(mut task) = self.tasks.remove(&tid) {
                                    let _ = task.prover.send(Message::Close(None)).await;
                                    for (_, mut p) in task.players.drain() {
                                        p.close(&send).await;
                                    }
                                    for (_, mut p) in task.viewers.drain() {
                                        p.close(&send).await;
                                    }
                                }
                            }
                            TextMessage::ClosePlayer(peer) => {
                                if let Some(task) = self.tasks.get_mut(&tid) {
                                    if let Some(mut player_sender) = task.players.remove(&peer) {
                                        player_sender.close(&send).await;
                                    }
                                }

                                let _ = self.peers.remove(&peer);
                            }
                            TextMessage::CloseViewer(peer) => {
                                if let Some(task) = self.tasks.get_mut(&tid) {
                                    if let Some(mut viewer_sender) = task.viewers.remove(&peer) {
                                        viewer_sender.close(&send).await;
                                    }
                                }

                                let _ = self.peers.remove(&peer);
                            }
                            TextMessage::Player(peer, text) => {
                                if let Some(task) = self.tasks.get_mut(&tid) {
                                    if let Some(ch) = task.players.get_mut(&peer) {
                                        ch.text(&send, text.clone()).await;
                                    }
                                    if let Some(ch) = task.viewers.get_mut(&peer) {
                                        ch.text(&send, text).await;
                                    }
                                }
                            }
                            TextMessage::Broadcast(text) => {
                                if let Some(task) = self.tasks.get_mut(&tid) {
                                    for (_, ch) in task.players.iter_mut() {
                                        ch.text(&send, text.clone()).await;
                                    }
                                    for (_, ch) in task.viewers.iter_mut() {
                                        ch.text(&send, text.clone()).await;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    P2pMessage::BinaryProver(tid, data) => match BinaryMessage::decode(data) {
                        BinaryMessage::Player(peer, data) => {
                            if let Some(task) = self.tasks.get_mut(&tid) {
                                if let Some(ch) = task.players.get_mut(&peer) {
                                    ch.binary(&send, data.clone()).await;
                                }
                                if let Some(ch) = task.viewers.get_mut(&peer) {
                                    ch.binary(&send, data).await;
                                }
                            }
                        }
                        BinaryMessage::Broadcast(data) => {
                            if let Some(task) = self.tasks.get_mut(&tid) {
                                for (_, ch) in task.players.iter_mut() {
                                    ch.binary(&send, data.clone()).await;
                                }
                                for (_, ch) in task.viewers.iter_mut() {
                                    ch.binary(&send, data.clone()).await;
                                }
                            }
                        }
                    },
                    P2pMessage::TextPlayer(tid, peer, text) => {
                        if let Some(task) = self.tasks.get_mut(&tid) {
                            if task.players.contains_key(&peer) {
                                let text = TextMessage::Player(peer, text).encode();
                                let _ = task.prover.send(Message::Text(text)).await;
                            }
                        }
                    }
                    P2pMessage::BinaryPlayer(tid, peer, data) => {
                        if let Some(task) = self.tasks.get_mut(&tid) {
                            if task.players.contains_key(&peer) {
                                let data = BinaryMessage::Player(peer, data).encode();
                                let _ = task.prover.send(Message::Binary(data)).await;
                            }
                        }
                    }
                },
                Some(P2pFuture::P2p(msg)) => match msg {
                    ReceiveMessage::StableConnect(node, mut data) => {
                        if data.len() < 8 {
                            continue;
                        }
                        let raw_data = hex::encode(data.split_off(8));
                        let mut tid_bytes = [0u8; 8];
                        tid_bytes.copy_from_slice(&data);
                        let tid = u64::from_le_bytes(tid_bytes);
                        let peer = Address::from_slice(node.id.as_bytes());

                        if let Some(task) = self.tasks.get_mut(&tid) {
                            if !task.started || task.players.contains_key(&peer) {
                                task.players.insert(peer, PlayerChannel::P2p(node.id));
                                let text = TextMessage::ConnectPlayer(peer, raw_data).encode();
                                let _ = task.prover.send(Message::Text(text)).await;

                                self.peers.insert(peer, tid);
                            } else if task.viewable {
                                task.viewers.insert(peer, PlayerChannel::P2p(node.id));
                                let text = TextMessage::ConnectViewer(peer).encode();
                                let _ = task.prover.send(Message::Text(text)).await;

                                self.peers.insert(peer, tid);
                            } else {
                                let _ = send
                                    .send(SendMessage::StableResult(0, node, false, false, vec![]))
                                    .await;
                            }
                        } else {
                            let _ = send
                                .send(SendMessage::StableResult(0, node, false, false, vec![]))
                                .await;
                        }
                    }
                    ReceiveMessage::StableLeave(node) => {
                        let peer = Address::from_slice(node.id.as_bytes());
                        let tid = self.peers.remove(&peer).unwrap_or(0);

                        if let Some(task) = self.tasks.get_mut(&tid) {
                            let text = if task.players.contains_key(&peer) {
                                if task.started {
                                    task.players.insert(peer, PlayerChannel::None);
                                } else {
                                    let _ = task.players.remove(&peer);
                                }
                                TextMessage::ClosePlayer(peer)
                            } else {
                                let _ = task.viewers.remove(&peer);
                                TextMessage::CloseViewer(peer)
                            };
                            let _ = task.prover.send(Message::Text(text.encode())).await;
                        }
                    }
                    ReceiveMessage::Data(node_id, data) => {
                        let peer = Address::from_slice(node_id.as_bytes());
                        let tid = self.peers.get(&peer).unwrap_or(&0);

                        if let Some(task) = self.tasks.get_mut(tid) {
                            if task.players.contains_key(&peer) {
                                let data = BinaryMessage::Player(peer, data).encode();
                                let _ = task.prover.send(Message::Binary(data)).await;
                            }
                        }
                    }
                    _ => {}
                },
                Some(P2pFuture::Clear) => {
                    let now = Utc::now().timestamp();

                    // clear task
                    let mut removed_tasks = vec![];
                    for (tid, task) in self.tasks.iter() {
                        if task.overtime < now {
                            removed_tasks.push(*tid);
                        }
                    }

                    for tid in removed_tasks {
                        if let Some(mut task) = self.tasks.remove(&tid) {
                            let _ = task.prover.send(Message::Close(None)).await;
                            for (_, mut p) in task.players.drain() {
                                p.close(&send).await;
                            }
                            for (_, mut p) in task.viewers.drain() {
                                p.close(&send).await;
                            }
                        }
                    }

                    // clear peers
                    let mut removed_peers = vec![];
                    for (pid, tid) in self.peers.iter() {
                        if self.tasks.contains_key(tid) {
                            removed_peers.push(*pid);
                        }
                    }
                    for pid in removed_peers {
                        let _ = self.peers.remove(&pid);
                    }
                }
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
