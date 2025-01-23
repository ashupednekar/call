use async_trait::async_trait;
use tokio::sync::broadcast::Sender;

mod nats;
mod channels;

pub struct Message{
    pub subject: String,
    pub data: Vec<u8> 
}

#[async_trait]
pub trait Broker{
    async fn produce(subject: &str, data: Vec<u8>);
    async fn consume(subject: &str, ch: Sender<Message>);
}
