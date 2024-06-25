use clap::{Args, Subcommand};

use crate::app::App;

use super::execute::Execute;

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
    async fn execute(self, app: App) -> anyhow::Result<()> {
        match self {
            Self::Get(command) => command.execute(app).await?,
            Self::Set(command) => command.execute(app).await?,
            Self::Del(command) => command.execute(app).await?,
        }
        Ok(())
    }
}

#[derive(Args, Debug)]
pub struct GetCommand {
    pub key: String,
}

impl Execute for GetCommand {
    async fn execute(self, app: App) -> anyhow::Result<()> {
        let value = app.json.get(self.key).await?;
        println!("{}", value);
        Ok(())
    }
}

#[derive(Args, Debug)]
pub struct SetCommand {
    pub key: String,
    pub value: String,
}

impl Execute for SetCommand {
    async fn execute(self, app: App) -> anyhow::Result<()> {
        app.json.set(self.key, self.value).await?;
        Ok(())
    }
}

#[derive(Args, Debug)]
pub struct DelCommand {
    pub key: String,
}

impl Execute for DelCommand {
    async fn execute(self, app: App) -> anyhow::Result<()> {
        app.json.del(self.key).await?;
        Ok(())
    }
}
