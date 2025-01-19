mod pkg;
mod cmd;
pub mod conf;
mod prelude;

use crate::prelude::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> { 
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()?)
        .with(tracing_subscriber::fmt::layer())
        .init();

    cmd::run().await?;

    Ok(())
}
