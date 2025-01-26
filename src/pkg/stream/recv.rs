use axum::extract::ws::{WebSocket, Message};
use futures::{stream::SplitSink, SinkExt};
use tokio::sync::broadcast::{self, Receiver};

use crate::prelude::Result;
use super::brokers::{Broker, Message as BrokerMessage};

pub async fn read_consumer_ch(
    mut sender: SplitSink<WebSocket, Message>,
    mut ch: Receiver<BrokerMessage>,
){
    while let Ok(msg) = ch.recv().await{
        if sender.send(Message::Binary(msg.data.into())).await.is_err(){
            tracing::error!("Failed to send message to WebSocket client");
        };
    }
}


pub async fn receive_broker_messages<T: Broker>(
    sender: SplitSink<WebSocket, Message>,
    broker: &T, 
    from: &str,
    to: &str
) -> Result<()> {
    let (tx, rx) = broadcast::channel(16);
    let subject = &format!("ws.{}.{}", &from, &to);
    tokio::select! {
        _ = broker.consume(subject, tx) => {},
        _ = read_consumer_ch(sender, rx) => {} 
    }
    tracing::info!("consumer closed");
    Ok(())
}
