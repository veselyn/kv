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
    pub async fn new() -> Result<Self, Error> {
        Self::builder().build().await
    }

    fn builder() -> Builder {
        Builder::default()
    }
}

#[derive(Debug, Default)]
pub struct Builder {
    config: Option<Config>,
    db: Option<Database>,
}

impl Builder {
    pub fn _config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    pub fn _db(mut self, db: Database) -> Self {
        self.db = Some(db);
        self
    }

    pub async fn build(self) -> Result<App, Error> {
        env_logger::init();

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
}
