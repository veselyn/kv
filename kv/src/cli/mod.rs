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
    pub async fn run(self) -> anyhow::Result<()> {
        let app = App::init().await?;

        match self.command {
            Command::Json(command) => command.execute(app).await?,
        }

        Ok(())
    }
}
