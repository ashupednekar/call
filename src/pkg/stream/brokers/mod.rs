use async_trait::async_trait;
use tokio::sync::broadcast::Sender;
use crate::prelude::Result;

pub mod nats;
pub mod inmemory;


#[derive(Debug, Clone)]
pub struct Message{
    pub subject: String,
    pub data: Vec<u8> 
}

#[async_trait]
pub trait Broker{
    async fn produce(&self, subject: &str, data: Vec<u8>) -> Result<()>;
    async fn consume(&self, subject: &str, ch: Sender<Message>) -> Result<()>;
}

