mod error;

use self::error::*;
use migration::MigratorTrait;
use sea_orm::{ConnectOptions, DatabaseConnection};

#[derive(Debug)]
pub struct Database {
    inner: DatabaseConnection,
}

impl Database {
    pub fn get(&self) -> &DatabaseConnection {
        &self.inner
    }

    pub async fn connect<C>(options: C) -> Result<Self, ConnectError>
    where
        C: Into<ConnectOptions>,
    {
        Ok(Self {
            inner: sea_orm::Database::connect(options).await?,
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

    pub async fn migrate<'c, C>(database: C) -> Result<(), MigrateError>
    where
        C: migration::IntoSchemaManagerConnection<'c>,
    {
        migration::Migrator::up(database, None).await?;
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
