#[derive(Debug)]
pub struct App {
    db: rusqlite::Connection,
}

impl App {
    pub fn new(db: rusqlite::Connection) -> Self {
        Self { db }
    }

    pub fn get<S>(&self, key: S) -> anyhow::Result<String>
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

    pub fn set<S>(&self, key: S, value: S) -> anyhow::Result<()>
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

    pub fn del<S>(&self, key: S) -> anyhow::Result<()>
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embedded;
    use pretty_assertions::assert_eq;

    impl Default for App {
        fn default() -> Self {
            let mut db = rusqlite::Connection::open_in_memory().expect("opening sqlite in memory");

            embedded::migrations::runner()
                .run(&mut db)
                .expect("migrating database");

            Self { db }
        }
    }

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
