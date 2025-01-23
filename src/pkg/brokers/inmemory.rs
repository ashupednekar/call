use std::collections::HashMap;

use async_trait::async_trait;
use tokio::sync::{Mutex, broadcast::{self, Receiver, Sender}};
use crate::prelude::Result;
use std::sync::Arc;

use super::{Broker, Message};


#[derive(Clone)]
struct InMemoryPubSub{
    pub channels: Arc<Mutex<HashMap<String, (Sender<Message>, Receiver<Message>)>>>
}

impl InMemoryPubSub{
    fn new() -> Self{
        Self{channels: Arc::new(Mutex::new(HashMap::new()))}
    }

    async fn get_or_create_channel(&self, subject: &str) -> (Sender<Message>, Receiver<Message>) {
        let mut channels = self.channels.lock().await;
        match &channels.get(subject){
            Some((tx, rx)) => (tx, rx),
            None => {
                let (tx, rx) = broadcast::channel(16);
                channels.insert(subject.to_string(), (tx, rx));
                (tx, rx)
            } 
        }

    }

}

#[async_trait]
impl Broker for InMemoryPubSub{


    async fn produce(&self, subject: &str, data: Vec<u8>) -> Result<()>{
        let (tx, _) = self.get_or_create_channel(subject).await;
        tx.send(Message{subject: subject.to_string(), data})?;
        Ok(())
    }

    async fn consume(&self, subject: &str, ch: Sender<Message>) -> Result<()>{
        let (_, mut rx) = self.get_or_create_channel(subject).await;
        while let Ok(msg) = rx.recv().await{
            ch.send(msg); 
        }
        Ok(())
    }
    
}


#[cfg(test)]
mod tests{
    use tokio::sync::broadcast;

    use super::*;

    #[tokio::test]
    async fn test_pubsub() -> Result<()>{
        let broker = InMemoryPubSub::new();

        async fn do_produce<T: Broker>(broker: &T) -> Result<()>{
            for i in 1..100{
                broker.produce("foo.bar", format!("sample data number {}", &i).into_bytes()).await?;
            }
            Ok(())
        }

        let (tx, mut rx) = broadcast::channel(16);

        tokio::select! {
            _ = do_produce(&broker) => {},
            _ = broker.consume("foo.bar", tx) => {}
        }

        while let Ok(msg) = rx.recv().await{
            tracing::debug!("msg: {:?}", &msg)
        }

        Ok(())
    }
}
