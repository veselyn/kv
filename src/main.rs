mod app;
mod cli;
mod migrations;

use anyhow::Context;
use app::App;
use clap::Parser;
use cli::{Cli, Command};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let data_dir = dirs::data_dir().context("getting data directory")?;
    let db_dir = data_dir.join("kv");
    std::fs::create_dir_all(&db_dir)?;
    let db_path = db_dir.join("db");

    let mut db = rusqlite::Connection::open(db_path)?;
    migrations::run(&mut db)?;

    let app = App::new(db);

    match cli.command {
        Command::Get { key } => {
            let value = app.get(key)?;
            println!("{}", value);
        }
        Command::Set { key, value } => {
            app.set(key, value)?;
        }
        Command::Del { key } => {
            app.del(key)?;
        }
    }

    Ok(())
}
