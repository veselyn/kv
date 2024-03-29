use clap::{Parser, Subcommand};

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

    let db = rusqlite::Connection::open("./kv.db")?;

    db.execute(
        "
        CREATE TABLE IF NOT EXISTS store (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )
        ",
        [],
    )?;

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
