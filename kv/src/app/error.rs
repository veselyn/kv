use std::path::PathBuf;

use thiserror::Error;

use crate::database;

#[derive(Debug, Error)]
pub enum NewError {
    #[error("creating config: {0}")]
    CreateConfig(#[from] ConfigError),
    #[error("invalid db path: {0}")]
    InvalidDbPath(PathBuf),
    #[error("creating kv dir in data dir: {0}")]
    CreateKvDir(#[source] std::io::Error),
    #[error("creating db file in kv dir: {0}")]
    CreateDbFile(#[source] std::io::Error),
    #[error("initializing database: {0}")]
    Database(#[from] database::Error),
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("getting os specific data dir")]
    GetDataDir,
}
