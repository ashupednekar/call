mod pkg;
mod cmd;
pub mod conf;
mod prelude;

use crate::prelude::Result;

#[tokio::main]
async fn main() -> Result<()> { 

    tracing_subscriber::fmt::init();
    
    cmd::run().await?;
    Ok(())
}
