use std::sync::Arc;

use tokio::sync::{Mutex, broadcast};

use crate::prelude::Result;
use super::brokers::Broker;


pub async fn receive_broker_messages<T: Broker + Send>(broker: T, user: &str) -> Result<()> {
    let (tx, mut rx) = broadcast::channel(16);
    let tx = Arc::new(Mutex::new(tx));
    let fut = broker.consume(&format!("ws.recv.{}", &user), tx);
    tokio::spawn(fut);
    Ok(())
}
