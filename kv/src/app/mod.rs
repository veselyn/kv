mod error;

use crate::{database::Database, json};
pub use error::*;

#[derive(Debug)]
pub struct App {
    pub json: json::Service,
}

impl App {
    pub async fn new() -> Result<Self, Error> {
        env_logger::init();

        let data_dir = dirs::data_dir().ok_or(Error::GetDataDir)?;

        let db_dir = data_dir.join("kv");
        std::fs::create_dir_all(&db_dir).map_err(Error::CreateKvDir)?;

        let db_path = db_dir.join("db");
        std::fs::File::options()
            .create(true)
            .truncate(false)
            .append(true)
            .open(&db_path)
            .map_err(Error::CreateDbFile)?;

        let db_path = format!("sqlite://{}", db_path.display());

        let db = Database::new(db_path).await?;

        Ok(Self {
            json: json::Service::new(json::Repository::new(db)),
        })
    }
}
