use futures::{SinkExt, StreamExt};
use async_nats::HeaderMap;
use axum::{extract::ws::{WebSocket, WebSocketUpgrade}, response::IntoResponse};


pub async fn handle_ws(
    ws: WebSocketUpgrade,
    _headers: HeaderMap,
) -> impl IntoResponse{
    ws.on_upgrade(move |stream| handle_connection(stream))
}

async fn handle_connection(
    stream: WebSocket
){
    let (mut sender, _receiver) = stream.split();
    if sender.send("ws conected".into()).await.is_err(){
        tracing::error!("couldn't send to client");
    }; 
}
