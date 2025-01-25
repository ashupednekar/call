use std::sync::Arc;

use tokio::sync::{Mutex, broadcast::{self, Receiver}};

use crate::prelude::Result;
use super::brokers::{Broker, Message};

pub async fn read_consumer_ch(mut ch: Receiver<Message>){
    while let Ok(msg) = ch.recv().await{

    }
}


pub async fn receive_broker_messages<T: Broker>(broker: T, user: &str) -> Result<()> {
    let (tx, rx) = broadcast::channel(16);
    let subject = &format!("ws.recv.{}", &user);
    tokio::select! {
        _ = broker.consume(subject, tx) => {},
        _ = read_consumer_ch(rx) => {} 
    }
    Ok(())
}
