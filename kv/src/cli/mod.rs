mod command;
mod json;

use crate::app::App;
use clap::{Parser, Subcommand};
use command::Execute;
pub use command::Result;

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
        let app = match App::new().await {
            Ok(app) => app,
            Err(err) => return Err(command::Error::default().message(err.to_string())),
        };

        self.run_with(app).await
    }

    pub async fn run_with(self, app: App) -> command::Result {
        match self.command {
            Command::Json(command) => command.execute(app).await,
        }
    }
}
