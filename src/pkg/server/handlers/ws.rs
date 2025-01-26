use axum_extra::extract::CookieJar;
use futures::{SinkExt, StreamExt};
use axum::{extract::ws::{WebSocket, WebSocketUpgrade}, http::HeaderMap, response::IntoResponse};

use crate::pkg::stream::{brokers::{inmemory::InMemoryPubSub, nats::NatsPubSub}, recv::receive_broker_messages, send::receive_client_messages};




pub async fn handle_ws(
    ws: WebSocketUpgrade,
    headers: HeaderMap,
) -> impl IntoResponse{

    let user = CookieJar::from_headers(&headers)
        .get("user")
        .filter(|c| !c.value().is_empty())
        .map(|c| c.value().to_owned())
        .expect("user cookie missing");
    //TODO: set cookie if not sent

    ws.on_upgrade(move |stream| async move {
        if let Err(e) = handle_connection(stream, user).await {
            tracing::error!("WebSocket connection error: {}", e);
        }
    })
}

async fn handle_connection(
    stream: WebSocket,
    user: String
) -> crate::prelude::Result<()>{
  
    let (sender, receiver) = stream.split();
    let broker = NatsPubSub::new().await?;

    tokio::select! {
        _ = receive_client_messages(receiver, &broker, &user) => {},
        _ = receive_broker_messages(sender, &broker, &user) => {}
    }

    Ok(())

}


