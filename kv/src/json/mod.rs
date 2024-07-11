mod error;
mod repository;

pub use error::*;
pub use repository::Repository;
use std::str::FromStr;

#[cfg_attr(test, derive(Default))]
#[derive(Debug)]
pub struct Service {
    repository: Repository,
}

impl Service {
    pub fn new(repository: Repository) -> Self {
        Self { repository }
    }

    pub async fn get<K>(
        &self,
        key: K,
        paths: Option<&[&str]>,
    ) -> Result<serde_json::Value, GetError>
    where
        K: Into<String>,
    {
        let key = key.into();

        if let Some(paths) = paths {
            let result = self
                .repository
                .get_paths(&key, paths)
                .await?
                .ok_or_else(|| GetError::KeyNotFound(key))?;

            let paths_not_found: Vec<String> = paths
                .iter()
                .cloned()
                .filter(|&path| !result.contains_key(path))
                .map(ToOwned::to_owned)
                .collect();

            if !paths_not_found.is_empty() {
                return Err(GetError::PathsNotFound(paths_not_found));
            }

            let map = serde_json::Map::from_iter(result.into_iter().map(|(path, value)| {
                (
                    path,
                    serde_json::Value::from_str(&value).expect("deserializing value"),
                )
            }));

            let value = serde_json::Value::Object(map);

            return Ok(value);
        }

        let result = self
            .repository
            .get(&key)
            .await?
            .ok_or_else(|| GetError::KeyNotFound(key))?;

        let value = serde_json::Value::from_str(&result).expect("deserializing value");

        Ok(value)
    }

    pub async fn set<K, V>(&self, key: K, value: V, path: Option<&str>) -> Result<(), SetError>
    where
        K: Into<String>,
        V: Into<String>,
    {
        let key = key.into();
        let value = value.into();

        if let Some(path) = path {
            let result =
                self.repository
                    .set_path(&key, &value, path)
                    .await
                    .map_err(|err| match err {
                        repository::SetError::MalformedJson(_) => SetError::InvalidJson(err),
                        repository::SetError::Other(_) => SetError::from(err),
                    })?;

            result.ok_or_else(|| SetError::KeyNotFound(key.clone()))?;

            return Ok(());
        }

        self.repository
            .set(key, value)
            .await
            .map_err(|err| match err {
                repository::SetError::MalformedJson(_) => SetError::InvalidJson(err),
                repository::SetError::Other(_) => SetError::from(err),
            })?;

        Ok(())
    }

    pub async fn del<K>(&self, key: K, _path: Option<&str>) -> Result<(), DelError>
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
