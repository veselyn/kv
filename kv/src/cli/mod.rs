mod command;
mod completion;
mod json;

use crate::{app::App, config::Config as AppConfig};
use clap::Args;
use clap::{Parser, Subcommand};
use command::Execute;
pub use command::Result;

#[derive(Parser, Debug, Clone)]
pub struct Cli {
    #[command(flatten)]
    pub config: Config,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Args, Debug, Clone)]
pub struct Config {
    #[arg(short, long, help = "Specify database to use")]
    pub database: Option<String>,
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
            AppConfig::try_from(self.clone())
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
