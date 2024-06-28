use super::config;
use crate::database;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("creating config: {0}")]
    Config(#[from] config::Error),
    #[error("initializing database: {0}")]
    Database(#[from] database::Error),
}
