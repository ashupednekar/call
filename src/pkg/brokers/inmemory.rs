use std::{borrow::BorrowMut, collections::HashMap};

use async_trait::async_trait;
use tokio::sync::{Mutex, broadcast::{self, Receiver, Sender}};
use std::sync::Arc;
use crate::prelude::Result;
use std::cell::RefCell;
use super::{Broker, Message};

type Map = Arc<Mutex<HashMap<String, (Sender<Message>, Receiver<Message>)>>>;


struct InMemoryPubSub{
    map: Map
}

impl InMemoryPubSub{
    fn new() -> Self{
        Self{map: Arc::new(Mutex::new(HashMap::new()))}
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

    async fn consume(&self, subject: &str, ch: Sender<Message>) -> Result<()>{
        let mut map = self.map.lock().await;
        let (_, ref mut rx) = map.entry(subject.to_string()).or_insert_with(||{
            let (tx, rx) = broadcast::channel(16);
            (tx, rx)
        });
        while let Ok(msg) = rx.recv().await{
            ch.send(msg);
        }; 
        Ok(())
    }
    
}


#[cfg(test)]
mod tests{
    use tokio::sync::broadcast;

    use super::*;

}
