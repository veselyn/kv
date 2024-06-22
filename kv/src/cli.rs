use clap::{Parser, Subcommand};

use crate::app::App;

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(subcommand, about = "Interact with JSON keys")]
    Json(JsonCommand),
}

#[derive(Subcommand, Debug)]
pub enum JsonCommand {
    #[command(about = "Get the value of a JSON key")]
    Get { key: String },
    #[command(about = "Set the value of a JSON key")]
    Set { key: String, value: String },
    #[command(about = "Delete the JSON key")]
    Del { key: String },
}

impl Cli {
    pub async fn run(self) -> anyhow::Result<()> {
        let app = App::init().await?;

        match self.command {
            Command::Json(json_command) => match json_command {
                JsonCommand::Get { key } => {
                    let value = app.json.get(key).await?;
                    println!("{}", value);
                }
                JsonCommand::Set { key, value } => {
                    app.json.set(key, value).await?;
                }
                JsonCommand::Del { key } => {
                    app.json.del(key).await?;
                }
            },
        }

        Ok(())
    }
}
