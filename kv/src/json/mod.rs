mod error;
mod format;
mod repository;

pub use error::*;
use format::format;
pub use repository::Repository;

#[cfg_attr(test, derive(Default))]
#[derive(Debug)]
pub struct Service {
    repository: Repository,
}

impl Service {
    pub fn new(repository: Repository) -> Self {
        Self { repository }
    }

    pub async fn get<S>(&self, key: S) -> Result<String, GetError>
    where
        S: Into<String>,
    {
        let key = key.into();

        let result = self.repository.get(&key).await?;
        let value = result.ok_or_else(|| GetError::KeyNotFound(key))?;
        let formatted = format(value)?;

        Ok(formatted)
    }

    pub async fn set<S>(&self, key: S, value: S) -> Result<(), SetError>
    where
        S: Into<String>,
    {
        self.repository.set(key, value).await?;

        Ok(())
    }

    pub async fn del<S>(&self, key: S) -> Result<(), DelError>
    where
        S: Into<String>,
    {
        self.repository.del(key).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests;
