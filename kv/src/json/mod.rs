mod error;
mod repository;

pub use error::*;
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

    pub async fn get<K>(&self, key: K) -> Result<String, GetError>
    where
        K: Into<String>,
    {
        let key = key.into();

        let result = self.repository.get(&key).await?;
        let value = result.ok_or_else(|| GetError::KeyNotFound(key))?;

        Ok(value)
    }

    pub async fn set<K, V>(&self, key: K, value: V) -> Result<(), SetError>
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.repository
            .set(key, value)
            .await
            .map_err(|err| match err {
                repository::SetError::MalformedJson(_) => SetError::InvalidJson(err),
                repository::SetError::Other(_) => SetError::from(err),
            })?;

        Ok(())
    }

    pub async fn del<K>(&self, key: K) -> Result<(), DelError>
    where
        K: Into<String>,
    {
        let key = key.into();

        let result = self.repository.del(&key).await?;
        result.ok_or_else(|| DelError::KeyNotFound(key))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests;
