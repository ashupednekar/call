use axum_extra::extract::CookieJar;
use futures::{SinkExt, StreamExt};
use axum::{extract::{Path, ws::{WebSocket, WebSocketUpgrade}}, http::HeaderMap, response::IntoResponse};

use crate::pkg::stream::{brokers::{inmemory::InMemoryPubSub, nats::NatsPubSub}, recv::receive_broker_messages, send::receive_client_messages};




pub async fn handle_ws(
    ws: WebSocketUpgrade,
    Path((from, to)): Path<(String, String)>
) -> impl IntoResponse{
    ws.on_upgrade(move |stream| async move {
        if let Err(e) = handle_connection(stream, from, to).await {
            tracing::error!("WebSocket connection error: {}", e);
        }
    })
}

async fn handle_connection(
    stream: WebSocket,
    from: String,
    to: String
) -> crate::prelude::Result<()>{
  
    let (sender, receiver) = stream.split();
    let broker = NatsPubSub::new().await?;

    tokio::select! {
        _ = receive_client_messages(receiver, &broker, &from, &to) => {},
        _ = receive_broker_messages(sender, &broker, &from, &to) => {}
    }

    Ok(())

}


