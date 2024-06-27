use super::command::{self, Execute};
use crate::app::App;
use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(about = "Get the value of a JSON key")]
    Get(GetCommand),
    #[command(about = "Set the value of a JSON key")]
    Set(SetCommand),
    #[command(about = "Delete the JSON key")]
    Del(DelCommand),
}

impl Execute for Command {
    async fn execute(self, app: App) -> command::Result {
        match self {
            Self::Get(command) => command.execute(app).await,
            Self::Set(command) => command.execute(app).await,
            Self::Del(command) => command.execute(app).await,
        }
    }
}

#[derive(Args, Debug)]
pub struct GetCommand {
    pub key: String,
}

impl Execute for GetCommand {
    async fn execute(self, app: App) -> command::Result {
        app.json
            .get(self.key)
            .await
            .map(|value| command::Output {
                stdout: Some(value),
                stderr: None,
            })
            .map_err(|err| command::Error::default().message(err.to_string()))
    }
}

#[derive(Args, Debug)]
pub struct SetCommand {
    pub key: String,
    pub value: String,
}

impl Execute for SetCommand {
    async fn execute(self, app: App) -> command::Result {
        app.json
            .set(self.key, self.value)
            .await
            .map(|_| command::Output::default())
            .map_err(|err| command::Error::default().message(err.to_string()))
    }
}

#[derive(Args, Debug)]
pub struct DelCommand {
    pub key: String,
}

impl Execute for DelCommand {
    async fn execute(self, app: App) -> command::Result {
        app.json
            .del(self.key)
            .await
            .map(|_| command::Output::default())
            .map_err(|err| command::Error::default().message(err.to_string()))
    }
}