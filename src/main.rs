use anyhow::Context;
use clap::{Parser, Subcommand};

mod embedded {
    refinery::embed_migrations!("./migrations");
}

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(about = "Get the value of a key")]
    Get { key: String },
    #[command(about = "Set the value of a key")]
    Set { key: String, value: String },
}

#[derive(Debug)]
struct App {
    db: rusqlite::Connection,
}

impl App {
    fn new(db: rusqlite::Connection) -> Self {
        Self { db }
    }

    fn get<S>(&self, key: S) -> anyhow::Result<String>
    where
        S: Into<String>,
    {
        Ok(self.db.query_row(
            "SELECT value FROM keys WHERE id = :key",
            rusqlite::named_params! {
                ":key": key.into()
            },
            |row| row.get("value"),
        )?)
    }

    fn set<S>(&self, key: S, value: S) -> anyhow::Result<()>
    where
        S: Into<String>,
    {
        self.db.execute(
            "INSERT INTO keys (id, value) VALUES (:key, :value)",
            rusqlite::named_params! {
                ":key": key.into(),
                ":value": value.into()
            },
        )?;
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let data_dir = dirs::data_dir().context("getting data directory")?;
    let db_dir = data_dir.join("kv");
    std::fs::create_dir_all(&db_dir)?;
    let db_path = db_dir.join("db");

    let mut db = rusqlite::Connection::open(db_path)?;
    embedded::migrations::runner().run(&mut db)?;

    let app = App::new(db);

    match cli.command {
        Command::Get { key } => {
            let value = app.get(key)?;
            println!("{}", value);
        }
        Command::Set { key, value } => {
            app.set(key, value)?;
        }
    }

    Ok(())
}
