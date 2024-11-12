use axum::extract::ws::{Message, WebSocket};
use axum::{
    extract::{Extension, Path, WebSocketUpgrade},
    http::{
        header::{HeaderMap, HeaderValue},
        StatusCode,
    },
    response::{IntoResponse, Response},
};
use chamomile::prelude::PeerId;
use chrono::prelude::Utc;
use ethers::types::{Signature, SignatureError};
use futures_util::{sink::SinkExt, StreamExt};
use pozk_db::Task;
use tokio::sync::mpsc::UnboundedSender;

use crate::app::AppContext;
use crate::p2p::P2pMessage;

/// player connect to the service
pub async fn player(
    Path(id): Path<u64>,
    mut headers: HeaderMap,
    ws: WebSocketUpgrade,
    Extension(app): Extension<AppContext>,
) -> Response {
    // check peer is valid
    let msg = format!("{}-{}", app.url, id);

    let peer_res = headers
        .remove("X-PEER")
        .unwrap_or(HeaderValue::from_static(""))
        .to_str()
        .unwrap_or("")
        .parse::<Signature>()
        .and_then(|s| s.recover(msg))
        .and_then(|s| PeerId::from_bytes(s.as_bytes()).map_err(|_| SignatureError::RecoveryError));

    let peer = match peer_res {
        Ok(s) => s,
        Err(_) => return (StatusCode::BAD_REQUEST, "invalid player").into_response(),
    };

    // check task id exists and player is valid
    if let Ok(Some(t)) = app.db.get::<Task>(&Task::to_key(id)) {
        // check task is active
        let now = Utc::now().timestamp();
        if t.is_me && !t.over && t.overtime > now {
            let sender = app.p2p_sender.clone();
            return ws.on_upgrade(move |socket: WebSocket| handle_prover(socket, id, sender));
        }
        let sender = app.p2p_sender.clone();
        return ws.on_upgrade(move |socket: WebSocket| handle_player(socket, id, peer, sender));
    }

    (StatusCode::BAD_REQUEST, "no task").into_response()
}

/// prover connect to the service
pub async fn prover(
    Path(id): Path<u64>,
    ws: WebSocketUpgrade,
    Extension(app): Extension<AppContext>,
) -> Response {
    // check task id exists
    if let Ok(Some(t)) = app.db.get::<Task>(&Task::to_key(id)) {
        // check task is active
        let now = Utc::now().timestamp();
        if t.is_me && !t.over && t.overtime > now {
            let sender = app.p2p_sender.clone();
            return ws.on_upgrade(move |socket: WebSocket| handle_prover(socket, id, sender));
        }
    }

    (StatusCode::BAD_REQUEST, "no task").into_response()
}

async fn handle_player(
    socket: WebSocket,
    id: u64,
    peer: PeerId,
    sender: UnboundedSender<P2pMessage>,
) {
    debug!("Websocket connected from player for task: {}", id);

    let (ws_sender, mut ws_receiver) = socket.split();

    // send sender to p2p service
    sender
        .send(P2pMessage::ConnectPlayer(id, peer, ws_sender))
        .expect("missing p2p service");

    while let Some(msg) = ws_receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                debug!("Receive text message from player");
                sender
                    .send(P2pMessage::TextPlayer(id, peer, text))
                    .expect("missing p2p service");
            }
            Ok(Message::Binary(data)) => {
                debug!("Receive binary message from player");
                sender
                    .send(P2pMessage::BinaryPlayer(id, peer, data))
                    .expect("missing p2p service");
            }
            Ok(Message::Close(_)) => {
                debug!("Player closed the WebSocket");
                sender
                    .send(P2pMessage::ClosePlayer(id, peer))
                    .expect("missing p2p service");
            }
            Ok(_) => {
                debug!("Received PING/PONG message from prover");
            }
            Err(_) => break,
        }
    }

    // send message to p2p service
    sender
        .send(P2pMessage::ClosePlayer(id, peer))
        .expect("missing p2p service");

    debug!("Player websocket closed for task: {}", id);
}

async fn handle_prover(socket: WebSocket, id: u64, sender: UnboundedSender<P2pMessage>) {
    debug!("WebSocket connected from prover for task: {}", id);

    let (ws_sender, mut ws_receiver) = socket.split();

    // register/replace websocket channel to p2p service
    sender
        .send(P2pMessage::ConnectProver(id, ws_sender))
        .expect("missing p2p service");

    while let Some(msg) = ws_receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                debug!("Receive text message from prover");
                sender
                    .send(P2pMessage::TextProver(id, text))
                    .expect("missing p2p service");
            }
            Ok(Message::Binary(data)) => {
                debug!("Receive binary message from prover");
                sender
                    .send(P2pMessage::BinaryProver(id, data))
                    .expect("missing p2p service");
            }
            Ok(Message::Close(_)) => {
                debug!("prover closed the WebSocket");
                sender
                    .send(P2pMessage::CloseProver(id))
                    .expect("missing p2p service");
            }
            Ok(_) => {
                debug!("Received PING/PONG message from prover");
            }
            Err(_) => break,
        }
    }

    // send message to p2p service
    sender
        .send(P2pMessage::CloseProver(id))
        .expect("missing p2p service");

    debug!("Prover websocket closed for task: {}", id);
}
