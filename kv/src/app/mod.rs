mod error;

use super::config::Config;
use crate::{database::Database, json};
pub use error::*;

#[derive(Debug)]
pub struct App {
    pub json: json::Service,
}

impl App {
    pub async fn new() -> Result<Self, Error> {
        Self::builder().build().await
    }

    pub fn builder() -> Builder {
        Builder::default()
    }
}

#[derive(Debug, Default)]
pub struct Builder {
    config: Option<Config>,
    db: Option<Database>,
}

impl Builder {
    pub async fn build(self) -> Result<App, Error> {
        let config = match self.config {
            Some(config) => config,
            None => Config::new()?,
        };

        let db = match self.db {
            Some(db) => db,
            None => Database::new(config.db_path).await?,
        };

        Ok(App {
            json: json::Service::new(json::Repository::new(db)),
        })
    }

    pub fn config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    pub fn db(mut self, db: Database) -> Self {
        self.db = Some(db);
        self
    }
}
