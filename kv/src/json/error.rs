use thiserror::Error;

use super::{format, repository};

#[derive(Debug, Error, PartialEq)]
pub enum GetError {
    #[error("key not found: {0}")]
    KeyNotFound(String),
    #[error("formatting json value: {0}")]
    Format(#[from] format::Error),
    #[error("getting key from repository: {0}")]
    Repository(#[from] repository::GetError),
}
