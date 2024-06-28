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

        let config = Config::new()?;

        let db = Database::new(config.db_path).await?;

        Ok(Self {
            json: json::Service::new(json::Repository::new(db)),
        })
    }
}
