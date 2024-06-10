#[derive(Debug)]
pub struct App {
    db: rusqlite::Connection,
}

impl App {
    pub fn new(db: rusqlite::Connection) -> Self {
        Self { db }
    }

    pub fn json_get<S>(&self, key: S) -> anyhow::Result<String>
    where
        S: Into<String>,
    {
        Ok(self.db.query_row(
            "SELECT value FROM keys WHERE id = :key and type = 'json'",
            rusqlite::named_params! {
                ":key": key.into()
            },
            |row| row.get("value"),
        )?)
    }

    pub fn json_set<S>(&self, key: S, value: S) -> anyhow::Result<()>
    where
        S: Into<String>,
    {
        self.db.execute(
            "INSERT INTO keys (id, type, value) VALUES (:key, 'json', :value)",
            rusqlite::named_params! {
                ":key": key.into(),
                ":value": value.into()
            },
        )?;
        Ok(())
    }

    pub fn json_del<S>(&self, key: S) -> anyhow::Result<()>
    where
        S: Into<String>,
    {
        self.db.execute(
            "DELETE FROM keys WHERE id = :key AND type = 'json'",
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
    use pretty_assertions::assert_eq;

    impl Default for App {
        fn default() -> Self {
            let mut db = rusqlite::Connection::open_in_memory().expect("opening sqlite in memory");

            crate::migrations::run(&mut db).expect("migrating database");

            Self { db }
        }
    }

    #[test]
    fn sets_and_gets_keys() -> anyhow::Result<()> {
        let app = App::default();

        assert_eq!(
            rusqlite::Error::QueryReturnedNoRows,
            app.json_get("key").unwrap_err().downcast()?
        );

        app.json_set("key", "value")?;

        assert_eq!("value", app.json_get("key")?);

        Ok(())
    }

    #[test]
    fn deletes_keys() -> anyhow::Result<()> {
        let app = App::default();

        app.json_set("key", "value")?;
        app.json_get("key")?;

        app.json_del("key")?;

        assert_eq!(
            rusqlite::Error::QueryReturnedNoRows,
            app.json_get("key").unwrap_err().downcast()?
        );

        Ok(())
    }
}
