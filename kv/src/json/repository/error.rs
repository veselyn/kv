use sea_orm::DbErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GetError {
    #[error(transparent)]
    Other(#[from] DbErr),
}

#[derive(Debug, Error)]
pub enum SetError {
    #[error("malformed json: {0}")]
    MalformedJson(#[source] DbErr),
    #[error(transparent)]
    Other(#[from] DbErr),
}

#[derive(Debug, Error)]
pub enum DelError {
    #[error(transparent)]
    Other(#[from] DbErr),
}
