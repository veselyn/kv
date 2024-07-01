mod command;
mod json;

use crate::{app::App, config::Config, env::Env};
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
        let env = Env::new();

        let config =
            Config::new().map_err(|err| command::Error::from(&env).message(err.to_string()))?;

        let app = App::builder()
            .env(env.clone())
            .config(config)
            .build()
            .await
            .map_err(|err| command::Error::from(env).message(err.to_string()))?;

        self.run_with(&app).await
    }

    pub async fn run_with(self, app: &App) -> command::Result {
        match self.command {
            Command::Json(command) => command.execute(app).await,
        }
    }
}
