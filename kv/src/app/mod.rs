mod config;
mod error;

use crate::{database::Database, json};
pub use config::*;
pub use error::*;

#[derive(Debug)]
pub struct App {
    pub json: json::Service,
}

impl App {
    pub async fn new() -> Result<Self, NewError> {
        env_logger::init();

        let config = Config::try_default()?;

        std::fs::create_dir_all(
            config
                .db_path
                .parent()
                .ok_or_else(|| NewError::InvalidDbPath(config.db_path.clone()))?,
        )
        .map_err(NewError::CreateKvDir)?;

        std::fs::File::options()
            .create(true)
            .truncate(false)
            .append(true)
            .open(&config.db_path)
            .map_err(NewError::CreateDbFile)?;

        let db_url = format!("sqlite://{}", config.db_path.display());

        let db = Database::connect_and_migrate(db_url).await?;

        Ok(Self {
            json: json::Service::new(json::Repository::new(db)),
        })
    }
}
