mod format;
mod repository;

use self::format::format;
pub use self::repository::*;

#[cfg_attr(test, derive(Default))]
#[derive(Debug)]
pub struct Service {
    repository: Repository,
}

impl Service {
    pub fn new(repository: Repository) -> Self {
        Self { repository }
    }

    pub async fn get<S>(&self, key: S) -> anyhow::Result<String>
    where
        S: Into<String>,
    {
        let result = self.repository.get(key).await?;
        let value = result.ok_or(anyhow::anyhow!("key does not exist"))?;
        let formatted = format(value)?;

        Ok(formatted)
    }

    pub async fn set<S>(&self, key: S, value: S) -> anyhow::Result<()>
    where
        S: Into<String>,
    {
        self.repository.set(key, value).await?;

        Ok(())
    }

    pub async fn del<S>(&self, key: S) -> anyhow::Result<()>
    where
        S: Into<String>,
    {
        self.repository.del(key).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests;
