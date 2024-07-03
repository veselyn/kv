mod error;

use crate::Cli;
pub use error::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub db_path: String,
}

impl Default for Config {
    fn default() -> Self {
        let data_dir = dirs::data_dir().expect("getting data dir for os ");

        Self {
            db_path: data_dir
                .join("kv")
                .join("db")
                .into_os_string()
                .into_string()
                .expect("db_path is not utf8"),
        }
    }
}

impl config::Source for Config {
    fn clone_into_box(&self) -> Box<dyn config::Source + Send + Sync> {
        todo!()
    }

    fn collect(&self) -> Result<config::Map<String, config::Value>, config::ConfigError> {
        let mut map = config::Map::new();

        map.insert(
            "db_path".to_owned(),
            config::Value::new(
                Some(&"default".to_owned()),
                config::ValueKind::String(self.db_path.to_owned()),
            ),
        );

        Ok(map)
    }
}

impl TryFrom<Cli> for Config {
    type Error = Error;

    fn try_from(cli: Cli) -> Result<Self, Self::Error> {
        let config_file = cli.config.clone().unwrap_or_else(|| {
            dirs::config_dir()
                .map(|dir| dir.join("kv").join("config"))
                .expect("getting config dir for os")
                .into_os_string()
                .into_string()
                .expect("config_file is not utf8")
        });

        let config = config::Config::builder()
            .add_source(Config::default())
            .add_source(config::File::with_name(&config_file).required(false))
            .add_source(config::Environment::with_prefix("KV"))
            .add_source(cli)
            .build()?;

        Ok(config.try_deserialize()?)
    }
}

impl config::Source for Cli {
    fn clone_into_box(&self) -> Box<dyn config::Source + Send + Sync> {
        Box::new(self.clone())
    }

    fn collect(&self) -> Result<config::Map<String, config::Value>, config::ConfigError> {
        Ok(config::Map::new())
    }
}
