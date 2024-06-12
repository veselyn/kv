mod jq;
#[cfg(test)]
mod tests;

use crate::app::App;

impl App {
    pub fn json_get<S>(&self, key: S) -> anyhow::Result<String>
    where
        S: Into<String>,
    {
        let value: String = self.db.query_row(
            "SELECT json(value) as value FROM keys WHERE id = :key AND type = 'json'",
            rusqlite::named_params! {
                ":key": key.into()
            },
            |row| row.get("value"),
        )?;

        let formatted = jq::format(value)?;

        Ok(formatted)
    }

    pub fn json_set<S>(&self, key: S, value: S) -> anyhow::Result<()>
    where
        S: Into<String>,
    {
        self.db.execute(
            "INSERT OR REPLACE INTO keys (id, type, value) VALUES (:key, 'json', json(:value))",
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
