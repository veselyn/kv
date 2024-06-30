mod error;

use super::config::Config;
use crate::{database::Database, env::Env, json};
pub use error::*;

#[derive(Debug)]
pub struct App {
    pub json: json::Service,
    env: Env,
    config: Config,
}

impl App {
    pub async fn new() -> Result<Self, Error> {
        Self::builder().build().await
    }

    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn env(&self) -> &Env {
        &self.env
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[derive(Debug, Default)]
pub struct Builder {
    env: Option<Env>,
    config: Option<Config>,
    db: Option<Database>,
}

impl Builder {
    pub async fn build(self) -> Result<App, Error> {
        let env = self.env.unwrap_or_else(Env::new);

        let config = match self.config {
            Some(config) => config,
            None => Config::new()?,
        };

        let db = match self.db {
            Some(db) => db,
            None => Database::new(&config.db_path).await?,
        };

        Ok(App {
            json: json::Service::new(json::Repository::new(db)),
            env,
            config,
        })
    }

    pub fn env(mut self, env: Env) -> Self {
        self.env = Some(env);
        self
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
