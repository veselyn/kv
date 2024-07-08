use super::command::{self, Execute};
use crate::app::App;
use crate::jq;
use crate::json::{DelError, GetError, SetError};
use clap::{Args, Subcommand};
use std::io::Cursor;

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    #[command(about = "Get the value of a JSON key")]
    Get(GetCommand),
    #[command(about = "Set the value of a JSON key")]
    Set(SetCommand),
    #[command(about = "Delete the JSON key")]
    Del(DelCommand),
}

impl Execute for Command {
    async fn execute(self, app: &App) -> command::Result {
        match self {
            Self::Get(command) => command.execute(app).await,
            Self::Set(command) => command.execute(app).await,
            Self::Del(command) => command.execute(app).await,
        }
    }
}

#[derive(Args, Debug, Clone)]
pub struct GetCommand {
    pub key: String,
}

impl Execute for GetCommand {
    async fn execute(self, app: &App) -> command::Result {
        app.json
            .get(self.key)
            .await
            .map(|value| -> command::Result {
                let formatted = jq::format(value).map_err(|err| {
                    command::Error::default().message(format!("formatting value: {}", err))
                })?;

                let output = command::Output::default().stdout(Cursor::new(formatted));

                Ok(output)
            })
            .map_err(|err| {
                command::Error::default().message(match err {
                    GetError::KeyNotFound(key) => {
                        format!("key {:?} not found", key)
                    }
                    GetError::Repository(_) => err.to_string(),
                })
            })?
    }
}

#[derive(Args, Debug, Clone)]
pub struct SetCommand {
    pub key: String,
    pub value: String,
}

impl Execute for SetCommand {
    async fn execute(self, app: &App) -> command::Result {
        app.json
            .set(self.key, self.value)
            .await
            .map(|_| command::Output::default())
            .map_err(|err| {
                command::Error::default().message(match err {
                    SetError::InvalidJson(_) => "invalid JSON".to_owned(),
                    SetError::Repository(_) => err.to_string(),
                })
            })
    }
}

#[derive(Args, Debug, Clone)]
pub struct DelCommand {
    pub key: String,
}

impl Execute for DelCommand {
    async fn execute(self, app: &App) -> command::Result {
        app.json
            .del(self.key)
            .await
            .map(|_| command::Output::default())
            .map_err(|err| {
                command::Error::default().message(match err {
                    DelError::KeyNotFound(key) => {
                        format!("key {:?} not found", key)
                    }
                    DelError::Repository(_) => err.to_string(),
                })
            })
    }
}
