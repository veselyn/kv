use thiserror::Error;

use crate::database;

#[derive(Debug, Error)]
pub enum Error {
    #[error("initializing logger: {0}")]
    Logger(#[from] log::SetLoggerError),
    #[error("getting os specific data dir")]
    GetDataDir,
    #[error("creating kv dir in data dir: {0}")]
    CreateKvDir(#[source] std::io::Error),
    #[error("creating db file in kv dir: {0}")]
    CreateDbFile(#[source] std::io::Error),
    #[error("initializing database: {0}")]
    Database(#[from] database::Error),
}
