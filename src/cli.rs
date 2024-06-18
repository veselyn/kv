use anyhow::Context;
use clap::{Parser, Subcommand};

use crate::{app::App, database, migrations};

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
        env_logger::init();

        let data_dir = dirs::data_dir().context("getting data directory")?;
        let db_dir = data_dir.join("kv");
        std::fs::create_dir_all(&db_dir)?;
        let db_path = db_dir.join("db");
        std::fs::File::options()
            .create(true)
            .truncate(false)
            .append(true)
            .open(&db_path)
            .expect("yes");
        let db_url = format!("sqlite://{}", db_path.display());

        let db = database::new(db_url).await?;
        migrations::run(&db).await?;

        let app = App::new(db);

        match self.command {
            Command::Json(json_command) => match json_command {
                JsonCommand::Get { key } => {
                    let value = app.json_get(key).await?;
                    println!("{}", value);
                }
                JsonCommand::Set { key, value } => {
                    app.json_set(key, value).await?;
                }
                JsonCommand::Del { key } => {
                    app.json_del(key).await?;
                }
            },
        }

        Ok(())
    }
}
