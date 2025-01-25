use std::collections::HashMap;

use async_trait::async_trait;
use tokio::sync::{Mutex, broadcast::{self, Receiver, Sender}};
use std::sync::Arc;
use crate::prelude::Result;
use super::{Broker, Message};

type Map = Arc<Mutex<HashMap<String, (Sender<Message>, Receiver<Message>)>>>;


pub struct InMemoryPubSub{
    map: Map
}

impl InMemoryPubSub{
    pub async fn new() -> Result<Self>{
        Ok(Self{map: Arc::new(Mutex::new(HashMap::new()))})
    }
}

#[async_trait]
impl Broker for InMemoryPubSub{


    async fn produce(&self, subject: &str, data: Vec<u8>) -> Result<()>{
        let mut map = self.map.lock().await;
        let (tx, _) = map.entry(subject.to_string()).or_insert_with(||{
            let (tx, rx) = broadcast::channel(16);
            (tx, rx)
        });
        tx.send(Message{subject: subject.to_string(), data})?;
        Ok(())
    }

    async fn consume(&self, subject: &str, ch: Arc<Mutex<Sender<Message>>>) -> Result<()>{
        let mut map = self.map.lock().await;
        let (_, ref mut rx) = map.entry(subject.to_string()).or_insert_with(||{
            let (tx, rx) = broadcast::channel(16);
            (tx, rx)
        });
        let ch = ch.lock().await;
        while let Ok(msg) = rx.recv().await{
            ch.send(msg)?;
        }; 
        Ok(())
    }
    
}


#[cfg(test)]
mod tests{
    use tokio::sync::broadcast;

    use super::*;

    #[tokio::test]
    async fn test_pubsub() -> Result<()>{ 
        let broker = InMemoryPubSub::new().await?;

        async fn do_produce<T: Broker>(broker: &T) -> Result<()>{
            for i in 1..100{
                broker.produce("foo.bar", format!("sample data number {}", &i).into_bytes()).await?;
            }
            Ok(())
        }

        let (tx, mut rx) = broadcast::channel(16);

        tokio::select! {
            _ = do_produce(&broker) => {},
            _ = broker.consume("foo.bar", Arc::new(Mutex::new(tx))) => {}
        }

        while let Ok(msg) = rx.recv().await{
            tracing::debug!("msg: {:?}", &msg)
        }

        Ok(())
    }
}
