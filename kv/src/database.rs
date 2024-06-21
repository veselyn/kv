use migration::MigratorTrait;
use sea_orm::{ConnectOptions, DatabaseConnection};

pub struct Database;

impl Database {
    pub async fn connect<C>(options: C) -> anyhow::Result<DatabaseConnection>
    where
        C: Into<ConnectOptions>,
    {
        Ok(sea_orm::Database::connect(options).await?)
    }

    pub async fn migrate<'c, C>(database: C) -> anyhow::Result<()>
    where
        C: migration::IntoSchemaManagerConnection<'c>,
    {
        migration::Migrator::up(database, None).await?;
        Ok(())
    }
}
