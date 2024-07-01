use super::command::{self, Execute};
use crate::app::App;
use crate::jq;
use crate::json::{DelError, GetError, SetError};
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
    async fn execute(self, app: &App) -> command::Result {
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
    async fn execute(self, app: &App) -> command::Result {
        app.json
            .get(self.key)
            .await
            .map(|value| -> command::Result {
                let formatted = jq::format(value).map_err(|err| {
                    command::Error::from(app.env()).message(format!("formatting value: {}", err))
                })?;

                Ok(command::Output::from(app.env()).stdout(formatted))
            })
            .map_err(|err| {
                command::Error::from(app.env()).message(match err {
                    GetError::KeyNotFound(key) => {
                        format!(r#"key "{}" not found"#, key)
                    }
                    GetError::Repository(_) => err.to_string(),
                })
            })?
    }
}

#[derive(Args, Debug)]
pub struct SetCommand {
    pub key: String,
    pub value: String,
}

impl Execute for SetCommand {
    async fn execute(self, app: &App) -> command::Result {
        app.json
            .set(self.key, self.value)
            .await
            .map(|_| command::Output::from(app.env()))
            .map_err(|err| {
                command::Error::from(app.env()).message(match err {
                    SetError::InvalidJson(_) => "invalid JSON".to_owned(),
                    SetError::Repository(_) => err.to_string(),
                })
            })
    }
}

#[derive(Args, Debug)]
pub struct DelCommand {
    pub key: String,
}

impl Execute for DelCommand {
    async fn execute(self, app: &App) -> command::Result {
        app.json
            .del(self.key)
            .await
            .map(|_| command::Output::from(app.env()))
            .map_err(|err| {
                command::Error::from(app.env()).message(match err {
                    DelError::KeyNotFound(key) => {
                        format!(r#"key "{}" not found"#, key)
                    }
                    DelError::Repository(_) => err.to_string(),
                })
            })
    }
}
