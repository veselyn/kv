use sea_orm::DbErr;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Create(#[from] CreateError),
    #[error(transparent)]
    Connect(#[from] ConnectError),
    #[error(transparent)]
    Migrate(#[from] MigrateError),
}

#[derive(Debug, Error)]
#[error("creating database: {0}")]
pub struct CreateError(#[from] pub io::Error);

#[derive(Debug, Error)]
#[error("connecting to database: {0}")]
pub struct ConnectError(#[from] pub DbErr);

#[derive(Debug, Error)]
#[error("migrating database: {0}")]
pub struct MigrateError(#[from] pub DbErr);
