use crate::database;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("creating config: {0}")]
    Config(#[from] ConfigError),
    #[error("initializing database: {0}")]
    Database(#[from] database::Error),
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("getting os specific data dir")]
    GetDataDir,
}
