use sea_orm::DbErr;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error(transparent)]
    Connect(#[from] ConnectError),
    #[error(transparent)]
    Migrate(#[from] MigrateError),
}

#[derive(Debug, Error, PartialEq)]
#[error("connecting to database: {0}")]
pub struct ConnectError(#[from] pub DbErr);

#[derive(Debug, Error, PartialEq)]
#[error("migrating database: {0}")]
pub struct MigrateError(#[from] pub DbErr);
