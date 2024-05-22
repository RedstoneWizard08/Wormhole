use std::net::SocketAddr;

use api::EVENT_BUS;
use axum::{
    debug_handler,
    extract::ConnectInfo,
    http::{header::USER_AGENT, HeaderMap},
    response::IntoResponse,
};
use axumite::{socket::WebSocket, upgrade::WebSocketUpgrade};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio_tungstenite::tungstenite::Message;

#[debug_handler]
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = headers
        .get(USER_AGENT)
        .map(|v| v.to_str().unwrap())
        .unwrap_or("Unknown browser");

    debug!("{} connected. (UA: {})", addr, user_agent);

    ws.on_upgrade(async move |socket: WebSocket| {
        let (mut tx, mut rx) = socket.split();
        let mut close = false;

        let tx_task = tokio::spawn(async move {
            while let Ok(item) = EVENT_BUS.1.recv() {
                if *&close {
                    break;
                }

                tx.send(Message::Text(
                    serde_json::to_string(&json!({
                        "event": "progress_callback",
                        "data": item,
                    }))
                    .unwrap(),
                ))
                .await
                .unwrap();
            }
        });

        let rx_task = tokio::spawn(async move {
            while let Some(Ok(item)) = rx.next().await {
                match item {
                    Message::Close(_) => {
                        *&mut close = true;
                        break;
                    }

                    _ => (),
                }
            }
        });

        tx_task.await.unwrap();
        rx_task.await.unwrap();
    })
}
