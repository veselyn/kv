use crate::database::Database;
use anyhow::Context;
use sea_orm::*;

mod json;

#[derive(Debug)]
pub struct App {
    db: DatabaseConnection,
}

impl App {
    pub async fn init() -> anyhow::Result<Self> {
        env_logger::init();

        let data_dir = dirs::data_dir().context("getting data directory")?;
        let db_dir = data_dir.join("kv");
        std::fs::create_dir_all(&db_dir)?;
        let db_path = db_dir.join("db");
        std::fs::File::options()
            .create(true)
            .truncate(false)
            .append(true)
            .open(&db_path)
            .expect("yes");
        let db_url = format!("sqlite://{}", db_path.display());

        let db = Database::connect(db_url).await?;
        Database::migrate(&db).await?;

        Ok(Self { db })
    }
}

#[cfg(test)]
impl Default for App {
    fn default() -> Self {
        let db = async_std::task::block_on(async {
            let db = Database::connect("sqlite::memory:")
                .await
                .expect("opening sqlite in memory");

            Database::migrate(&db).await.expect("migrating database");

            db
        });

        Self { db }
    }
}
