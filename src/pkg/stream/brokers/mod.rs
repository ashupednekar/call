use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::{Mutex, broadcast::Sender};
use crate::{conf::{settings, Brokers}, prelude::Result};

mod nats;
mod inmemory;

use nats::NatsPubSub;
use inmemory::InMemoryPubSub;

#[derive(Debug, Clone)]
pub struct Message{
    pub subject: String,
    pub data: Vec<u8> 
}

#[async_trait]
pub trait Broker{
    async fn produce(&self, subject: &str, data: Vec<u8>) -> Result<()>;
    async fn consume(&self, subject: &str, ch: Arc<Mutex<Sender<Message>>>) -> Result<()>;
}

async fn new_broker<T>() -> Result<Box<dyn Broker>> {
    match settings.broker{
        Brokers::Nats => { 
            let broker: NatsPubSub = NatsPubSub::new().await?;
            Ok(Box::new(broker))
        },
        Brokers::InMemory => { 
            let broker: InMemoryPubSub = InMemoryPubSub::new().await?;
            Ok(Box::new(broker))
        },
    }
}
