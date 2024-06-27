mod command;
mod json;

use crate::app::App;
use clap::{Parser, Subcommand};
use command::Execute;

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(subcommand, about = "Interact with JSON keys")]
    Json(json::Command),
}

impl Cli {
    pub async fn run(self) -> command::Result {
        let app = match App::init().await {
            Ok(app) => app,
            Err(err) => return Err(command::Error::default().message(err.to_string())),
        };

        match self.command {
            Command::Json(command) => command.execute(app).await,
        }
    }
}
