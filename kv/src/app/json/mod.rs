mod format;
mod repository;
#[cfg(test)]
mod tests;

pub use repository::Repository;

use crate::app::App;

use self::format::format;

impl App {
    pub async fn json_get<S>(&self, key: S) -> anyhow::Result<String>
    where
        S: Into<String>,
    {
        let result = self.json_repository.get(key).await?;
        let value = result.ok_or(anyhow::anyhow!("key does not exist"))?;
        let formatted = format(value)?;

        Ok(formatted)
    }

    pub async fn json_set<S>(&self, key: S, value: S) -> anyhow::Result<()>
    where
        S: Into<String>,
    {
        self.json_repository.set(key, value).await?;

        Ok(())
    }

    pub async fn json_del<S>(&self, key: S) -> anyhow::Result<()>
    where
        S: Into<String>,
    {
        self.json_repository.del(key).await?;

        Ok(())
    }
}
