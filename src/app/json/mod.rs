#[cfg(test)]
mod tests;

use crate::app::App;

impl App {
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
            "INSERT INTO keys (id, type, value) VALUES (:key, 'json', json(:value))",
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
