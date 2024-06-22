use migration::MigratorTrait;
use sea_orm::{ConnectOptions, DatabaseConnection, DbErr};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("connecting to database: {0}")]
    Connect(DbErr),
    #[error("migrating database: {0}")]
    Migrate(DbErr),
}

#[derive(Debug)]
pub struct Database {
    inner: DatabaseConnection,
}

impl Database {
    pub fn get(&self) -> &DatabaseConnection {
        &self.inner
    }

    pub async fn connect<C>(options: C) -> Result<Self, Error>
    where
        C: Into<ConnectOptions>,
    {
        Ok(Self {
            inner: sea_orm::Database::connect(options)
                .await
                .map_err(Error::Connect)?,
        })
    }

    pub async fn connect_and_migrate<C>(options: C) -> Result<Self, Error>
    where
        C: Into<ConnectOptions>,
    {
        let db = Self::connect(options).await?;
        Self::migrate(db.get()).await?;
        Ok(db)
    }

    pub async fn migrate<'c, C>(database: C) -> Result<(), Error>
    where
        C: migration::IntoSchemaManagerConnection<'c>,
    {
        migration::Migrator::up(database, None)
            .await
            .map_err(Error::Migrate)?;
        Ok(())
    }
}

#[cfg(test)]
impl Default for Database {
    fn default() -> Self {
        async_std::task::block_on(async {
            Database::connect_and_migrate("sqlite::memory:")
                .await
                .expect("opening sqlite in memory")
        })
    }
}
