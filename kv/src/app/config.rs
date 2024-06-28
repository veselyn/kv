use super::ConfigError;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub db_path: PathBuf,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let data_dir = dirs::data_dir().ok_or(ConfigError::GetDataDir)?;
        let db_dir = data_dir.join("kv");
        let db_path = db_dir.join("db");

        Ok(Self { db_path })
    }

    pub fn _db_path(mut self, db_path: PathBuf) -> Self {
        self.db_path = db_path;
        self
    }
}
