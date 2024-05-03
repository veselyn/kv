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
    #[command(about = "Delete the key")]
    Del { key: String },
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

    fn del<S>(&self, key: S) -> anyhow::Result<()>
    where
        S: Into<String>,
    {
        self.db.execute(
            "DELETE FROM keys WHERE id = :key",
            rusqlite::named_params! {
                ":key": key.into()
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
        Command::Del { key } => {
            app.del(key)?;
        }
    }

    Ok(())
}

#[cfg(test)]
impl Default for App {
    fn default() -> Self {
        let mut db = rusqlite::Connection::open_in_memory().expect("opening sqlite in memory");

        embedded::migrations::runner()
            .run(&mut db)
            .expect("migrating database");

        Self { db }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn sets_and_gets_keys() -> anyhow::Result<()> {
        let app = App::default();

        assert_eq!(
            rusqlite::Error::QueryReturnedNoRows,
            app.get("key").unwrap_err().downcast()?
        );

        app.set("key", "value")?;

        assert_eq!("value", app.get("key")?);

        Ok(())
    }

    #[test]
    fn deletes_keys() -> anyhow::Result<()> {
        let app = App::default();

        app.set("key", "value")?;
        app.get("key")?;

        app.del("key")?;

        assert_eq!(
            rusqlite::Error::QueryReturnedNoRows,
            app.get("key").unwrap_err().downcast()?
        );

        Ok(())
    }
}
