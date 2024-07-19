pub use assert_cmd::prelude::*;
use assert_cmd::{crate_name, Command};
pub use kv::Config;
use std::fmt::Display;
use tempfile::tempdir;

#[derive(Debug, Clone)]
pub struct Cli {
    config: Config,
}

impl Cli {
    pub fn new() -> Self {
        let config = Config {
            database: tempdir()
                .expect("creating temp dir")
                .into_path()
                .join("db")
                .to_str()
                .expect("creating database path")
                .to_owned(),
        };

        Self { config }
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
        let config = cli.config.clone();
        Box::new(move || {
            let mut cmd = Command::cargo_bin(crate_name!()).expect("creating command");
            cmd.env("KV_DATABASE", &config.database);
            cmd
        })
    }
}

pub trait Newline {
    fn nl(&self) -> String;
}

impl<T> Newline for T
where
    T: Into<String> + AsRef<str> + Display,
{
    fn nl(&self) -> String {
        format!("{self}\n")
    }
}
