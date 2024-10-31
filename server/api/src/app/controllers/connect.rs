use axum::{
    body::Bytes,
    extract::{Extension, Json, Path, WebSocketUpgrade},
};
use axum::extract::ws::{CloseFrame, Message, WebSocket};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio::{net::TcpStream, select};

use futures_util::{sink::SinkExt, StreamExt};

use crate::app::{success, AppContext, Error, Result};

/// player connect to the service
pub async fn player(Path(id): Path<u64>, ws: WebSocketUpgrade) -> Response {
    // check task id exists

    ws.on_upgrade(move |socket: WebSocket| {
        handle_player(socket, id)
    })
}

/// prover connect to the service
pub async fn prover(Path(id): Path<u64>, ws: WebSocketUpgrade) -> Response {
    // check task id exists

    ws.on_upgrade(move |socket: WebSocket| {
        handle_prover(socket, id)
    })
}

async fn handle_player(socket: WebSocket, id: u64) {
    debug!("Player websocket connected for task: {}", id);

    let (mut sender, mut receiver) = socket.split();

    while let Some(msg) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                // TODO match test

                debug!("Received text message from client");
                if let Err(e) = ws_connection.receive_text_msg(&text).await {
                    ws_connection.send_error_msg(e).await;
                }
            }
            Message::Binary(_) => {
                debug!("Receive binary message to client");
                ws_connection
                    .close_all(Some(Error::WebSocket(1310)))
                    .await?
            }
            Message::Close(_) => {
                debug!("Client closed the WebSocket");
                ws_connection.close_remote().await?
            }
            Message::Ping(data) => {
                debug!("Received PING message from client");
                ws_connection
                    .remote_socket
                    .send(TMessage::Ping(data))
                    .await
                    .map_err(|_| Error::WebSocket(1314))?;
            }
            Message::Pong(data) => {
                debug!("Received PONG message from client");
                ws_connection
                    .remote_socket
                    .send(TMessage::Pong(data))
                    .await
                    .map_err(|_| Error::WebSocket(1314))?;
            }
        }
    }

    debug!("Player websocket closed for task: {}", id);
}

async fn handle_prover(socket: WebSocket, id: String) {
    debug!("WebSocket connected for task: {}", id);

    // register/replace websocket channel to p2p service
    let (mut sender, mut receiver) = socket.split();

    while let Some(msg) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                debug!("Received text message from client");
                if let Err(e) = ws_connection.receive_text_msg(&text).await {
                    ws_connection.send_error_msg(e).await;
                }
            }
            Message::Binary(_) => {
                debug!("Receive binary message to client");
                ws_connection
                    .close_all(Some(Error::WebSocket(1310)))
                    .await?
            }
            Message::Close(_) => {
                debug!("Client closed the WebSocket");
                ws_connection.close_remote().await?
            }
            Message::Ping(data) => {
                debug!("Received PING message from client");
                ws_connection
                    .remote_socket
                    .send(TMessage::Ping(data))
                    .await
                    .map_err(|_| Error::WebSocket(1314))?;
            }
            Message::Pong(data) => {
                debug!("Received PONG message from client");
                ws_connection
                    .remote_socket
                    .send(TMessage::Pong(data))
                    .await
                    .map_err(|_| Error::WebSocket(1314))?;
            }
        }
    }

    debug!("Prover websocket closed for task: {}", id);
}
