use sea_orm::DbErr;
use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum GetError {
    #[error(transparent)]
    Other(#[from] DbErr),
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum SetError {
    #[error("malformed json: {0}")]
    MalformedJson(#[source] DbErr),
    #[error(transparent)]
    Other(#[from] DbErr),
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum DelError {
    #[error("key not found: {0:?}")]
    KeyNotFound(String),
    #[error(transparent)]
    Other(#[from] DbErr),
}
