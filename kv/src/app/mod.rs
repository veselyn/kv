mod error;

use crate::{config::Config, database::Database, json};
pub use error::*;
use std::path::PathBuf;

#[derive(Debug)]
pub struct App {
    pub json: json::Service,
}

impl App {
    pub async fn new(config: Config) -> Result<Self, Error> {
        std::fs::create_dir_all(
            PathBuf::from(&config.db_path)
                .parent()
                .expect("invalid db_path"),
        )
        .map_err(Error::CreateKvDir)?;

        std::fs::File::options()
            .create(true)
            .truncate(false)
            .append(true)
            .open(&config.db_path)
            .map_err(Error::CreateDbFile)?;

        let db = Database::new(&config.db_path).await?;

        Ok(Self {
            json: json::Service::new(json::Repository::new(db)),
        })
    }
}
