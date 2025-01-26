use async_nats::Client;
use async_trait::async_trait;
use serde::Deserialize;
use tokio_stream::StreamExt;
use tokio::sync::broadcast::Sender;
use crate::prelude::Result;
use config::{Config, Environment, ConfigError};

use super::{Broker, Message};

#[derive(Deserialize)]
struct Conf{
    nats_broker_url: String
}

impl Conf{
    pub fn new() -> core::result::Result<Self, ConfigError> {
        let conf = Config::builder()
            .add_source(Environment::default())
            .build()?;
        conf.try_deserialize()
    }
}


pub struct NatsPubSub{
    pub client: Client
}


impl NatsPubSub{
    pub async fn new() -> Result<Self>{
        let conf = Conf::new()?;
        let client = async_nats::connect(conf.nats_broker_url).await?;
        Ok(Self{client})
    }
}

#[async_trait]
impl Broker for NatsPubSub{
    async fn produce(&self, subject: &str, data: Vec<u8>) -> Result<()>{
        tracing::debug!("producing to: {}", &subject);
        self.client.publish(subject.to_string(), data.into())
            .await?; 
        Ok(())
    }

    async fn consume(&self, subject: &str, ch: Sender<Message>) -> Result<()>{
        tracing::debug!("subscribing at: {}", &subject);
        let mut subscriber = self.client.subscribe(subject.to_string())
            .await?;
        while let Some(msg) = subscriber.next().await{
            tracing::debug!("got msg from broker: {:?}", &msg);
            ch.send(Message{
                subject: subject.to_string(),
                data: msg.payload.to_vec()
            })?;
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
        let broker = NatsPubSub::new().await?;

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
