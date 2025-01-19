pub mod api;
pub mod app;
pub mod cli;

use crate::prelude::Result;
use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(about = "lets you start fullstack, api or cli mode")]
struct Cmd{
    #[command(subcommand)]
    command: Option<SubcommandType>
}

#[derive(Subcommand)]
enum SubcommandType{
    App,
    Api,
    Cli
}

pub async fn run() -> Result<()>{
    let args = Cmd::parse();

    match args.command{
        Some(SubcommandType::App) => {
            
        }
        Some(SubcommandType::Api) => {

        }
        Some(SubcommandType::Cli) => {

        }
        None => {
            tracing::error!("no subcommand passed");
        }
    }

    Ok(())
}


