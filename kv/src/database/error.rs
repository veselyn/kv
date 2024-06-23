use sea_orm::DbErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Connect(#[from] ConnectError),
    #[error(transparent)]
    Migrate(#[from] MigrateError),
}

#[derive(Debug, Error)]
#[error("connecting to database: {0}")]
pub struct ConnectError(#[from] pub DbErr);

#[derive(Debug, Error)]
#[error("migrating database: {0}")]
pub struct MigrateError(#[from] pub DbErr);
