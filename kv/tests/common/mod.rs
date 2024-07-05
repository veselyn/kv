pub use assert_cmd::prelude::*;
use assert_cmd::{crate_name, Command};
pub use kv::Config;
use std::fs::File;
use std::path::PathBuf;
use tempfile::tempdir;

#[derive(Debug, Clone)]
pub struct Cli {
    tmp_dir: PathBuf,
    config: Config,
}

impl Cli {
    pub fn new() -> Self {
        let tmp_dir = tempdir().expect("creating temp dir").into_path();

        let config = Config {
            database_path: tmp_dir
                .join("db")
                .to_str()
                .expect("creating database path")
                .to_owned(),
        };

        Self { tmp_dir, config }
    }

    pub fn config<F>(&mut self, config: F) -> &Self
    where
        F: Fn(&Config) -> Config,
    {
        self.config = config(&self.config);
        self
    }

    pub fn to_cmd(&self) -> Cmd {
        self.into()
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

pub type Cmd = Box<dyn Fn() -> Command>;

impl From<&Cli> for Cmd {
    fn from(cli: &Cli) -> Self {
        let config_file_path = cli.tmp_dir.join("config.json");
        let config_file = File::create(&config_file_path).expect("creating config file");

        serde_json::to_writer(config_file, &cli.config).expect("serializing config to file");

        let config = config_file_path
            .to_str()
            .expect("config file path is not utf8")
            .to_owned();

        Box::new(move || {
            let mut cmd = Command::cargo_bin(crate_name!()).expect("creating command");
            cmd.args(["--config", &config]);
            cmd
        })
    }
}
