use crate::{database::Database, json};
use anyhow::Context;

#[derive(Debug)]
pub struct App {
    pub json: json::Service,
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
            .open(&db_path)?;
        let db_url = format!("sqlite://{}", db_path.display());

        let db = Database::connect_and_migrate(db_url).await?;

        Ok(Self {
            json: json::Service::new(json::Repository::new(db)),
        })
    }
}
