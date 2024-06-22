use sea_orm::DbErr;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum GetError {
    #[error(transparent)]
    Other(#[from] DbErr),
}

#[derive(Debug, Error)]
pub enum SetError {
    #[error(transparent)]
    Other(#[from] DbErr),
}

#[derive(Debug, Error)]
pub enum DelError {
    #[error(transparent)]
    Other(#[from] DbErr),
}
