use crate::prelude::Result;
use axum::extract::ws::{Message, WebSocket};
use futures::{stream::SplitStream, StreamExt};

use super::brokers::Broker;


pub async fn receive_client_messages<T: Broker>(
    mut receiver: SplitStream<WebSocket>,
    broker: &T,
    from: &str,
    to: &str
) -> Result<()>{
    let subject = format!("ws.{}.{}", &to, &from);
    while let Some(Ok(msg)) = receiver.next().await{
        if let Message::Close(_) = msg{
            break; 
        }else{
            tracing::debug!("got msg from client: {:?}", &msg);
            broker.produce(&subject, msg.into()).await?;
        }
    }
    Ok(())
}
