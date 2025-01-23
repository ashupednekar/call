use async_nats::Client;
use async_trait::async_trait;
use serde::Deserialize;
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


struct NatsPubSub{
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
    async fn produce(subject: &str, data: Vec<u8>){

    }

    async fn consume(subject: &str, ch: Sender<Message>){

    }
}
