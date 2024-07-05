mod error;

pub use error::*;
use migration::MigratorTrait;
use sea_orm::DatabaseConnection;
use std::{io, ops::Deref, path::Path};

#[derive(Debug)]
pub struct Database {
    inner: DatabaseConnection,
}

impl Deref for Database {
    type Target = DatabaseConnection;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Database {
    pub async fn new<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        Self::create(&path)?;
        let database = Self::connect(&path).await?;
        Self::migrate(&database).await?;

        Ok(database)
    }

    pub fn create<P>(path: P) -> Result<(), CreateError>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        if path == Path::new(":memory:") {
            return Ok(());
        }

        std::fs::create_dir_all(
            path.parent()
                .ok_or_else(|| CreateError(io::ErrorKind::InvalidInput.into()))?,
        )
        .map_err(CreateError)?;

        std::fs::File::options()
            .create(true)
            .truncate(false)
            .append(true)
            .open(path)
            .map_err(CreateError)?;

        Ok(())
    }

    pub async fn connect<P>(path: P) -> Result<Self, ConnectError>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().to_str().expect("database path is not utf8");

        let uri = match path {
            ":memory:" => "sqlite::memory:".to_owned(),
            path => format!("sqlite://{path}"),
        };

        Ok(Self {
            inner: sea_orm::Database::connect(uri).await?,
        })
    }

    pub async fn migrate(database: &Self) -> Result<(), MigrateError> {
        migration::Migrator::up(&database.inner, None).await?;
        Ok(())
    }
}

#[cfg(test)]
impl Default for Database {
    fn default() -> Self {
        async_std::task::block_on(async {
            Database::new(":memory:")
                .await
                .expect("creating database in memory")
        })
    }
}
