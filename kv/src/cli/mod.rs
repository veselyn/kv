mod command;
mod completion;
mod json;

use crate::{app::App, config::Config};
use clap::{Parser, Subcommand};
use command::Execute;
pub use command::Result;

#[derive(Parser, Debug, Clone)]
pub struct Cli {
    #[arg(short, long, help = "Specify database to use")]
    pub database: Option<String>,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    #[command(subcommand, about = "Interact with JSON keys")]
    Json(json::Command),
    #[command(subcommand, about = "Generate completion script")]
    Completion(completion::Command),
}

impl Cli {
    pub async fn run(self) -> command::Result {
        let app = App::new(
            Config::try_from(self.clone())
                .map_err(|err| command::Error::default().message(err.to_string()))?,
        )
        .await
        .map_err(|err| command::Error::default().message(err.to_string()))?;

        match self.command {
            Command::Json(command) => command.execute(&app).await,
            Command::Completion(command) => command.execute(&app).await,
        }
    }
}
