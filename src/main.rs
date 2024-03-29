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

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let data_dir = dirs::data_dir().context("getting data directory")?;
    let db_dir = data_dir.join("kv");
    std::fs::create_dir_all(&db_dir)?;
    let db_path = db_dir.join("db");
    let mut db = rusqlite::Connection::open(db_path)?;

    embedded::migrations::runner().run(&mut db)?;

    match cli.command {
        Command::Get { key } => {
            let value: String = db.query_row(
                "SELECT value FROM store WHERE key = :key",
                rusqlite::named_params! {
                    ":key": key
                },
                |row| row.get("value"),
            )?;
            println!("{}", value);
        }
        Command::Set { key, value } => {
            db.execute(
                "INSERT INTO store (key, value) VALUES (:key, :value)",
                rusqlite::named_params! {
                    ":key": key,
                    ":value": value
                },
            )?;
        }
    }

    Ok(())
}
