mod error;

pub use error::*;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub db_path: PathBuf,
}

impl Config {
    pub fn new() -> Result<Self, Error> {
        Self::builder().build()
    }

    pub fn builder() -> Builder {
        Builder::default()
    }
}

#[derive(Debug, Default)]
pub struct Builder {
    pub db_path: Option<PathBuf>,
}

impl Builder {
    pub fn build(self) -> Result<Config, Error> {
        let db_path = match self.db_path {
            Some(db_path) => db_path,
            None => {
                let data_dir = dirs::data_dir().ok_or(Error::GetDataDir)?;
                let db_dir = data_dir.join("kv");
                db_dir.join("db")
            }
        };

        Ok(Config { db_path })
    }

    pub fn db_path(mut self, db_path: PathBuf) -> Self {
        self.db_path = Some(db_path);
        self
    }
}
