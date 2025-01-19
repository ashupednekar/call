pub mod api;
pub mod app;
pub mod cli;

use crate::{pkg::server::serve::listen, prelude::Result};
use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(about = "lets you start fullstack, api or cli mode")]
struct Cmd{
    #[command(subcommand)]
    command: Option<SubcommandType>
}

#[derive(Subcommand)]
enum SubcommandType{
    Listen,
    Cli
}

pub async fn run() -> Result<()>{
    let args = Cmd::parse();

    match args.command{
        Some(SubcommandType::Listen) => {
            listen().await?;
        }
        Some(SubcommandType::Cli) => {

        }
        None => {
            tracing::error!("no subcommand passed");
        }
    }

    Ok(())
}


