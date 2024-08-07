use thiserror::Error;

use super::repository;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum GetError {
    #[error("key not found: {0:?}")]
    KeyNotFound(String),
    #[error("paths not found: {0:?}")]
    PathsNotFound(Vec<String>),
    #[error("getting key from repository: {0}")]
    Repository(#[from] repository::GetError),
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum SetError {
    #[error("key not found: {0:?}")]
    KeyNotFound(String),
    #[error("received invalid json")]
    InvalidJson(#[source] repository::SetError),
    #[error("setting key into repository: {0}")]
    Repository(#[from] repository::SetError),
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum DelError {
    #[error("key not found: {0:?}")]
    KeyNotFound(String),
    #[error("path not found: {0:?}")]
    PathNotFound(String),
    #[error("deleting key from repository: {0}")]
    Repository(#[from] repository::DelError),
}
