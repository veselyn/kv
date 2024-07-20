pub use assert_cmd::prelude::*;
use assert_cmd::{crate_name, Command};
pub use kv::{Config, ConfigBuilder};
use std::{collections::HashMap, fmt::Display};
use tempfile::tempdir;

#[derive(Debug, Clone)]
pub struct Cli {
    config: ConfigBuilder,
}

impl Cli {
    pub fn new() -> Self {
        let config_builder = Config::builder()
            .database(
                tempdir()
                    .expect("creating temp dir")
                    .into_path()
                    .join("db")
                    .to_str()
                    .expect("creating database path")
                    .to_owned(),
            )
            .to_owned();

        Self {
            config: config_builder,
        }
    }

    pub fn config<F>(&mut self, mut apply: F) -> &Self
    where
        F: FnMut(&mut ConfigBuilder),
    {
        apply(&mut self.config);
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
        let config = &cli.config;

        let mut envs = HashMap::new();

        envs.insert(
            "KV_DATABASE".to_owned(),
            config
                .database
                .as_ref()
                .expect("database should be changed in tests")
                .to_owned(),
        );

        Box::new(move || {
            let mut cmd = Command::cargo_bin(crate_name!()).expect("creating command");
            cmd.envs(&envs);
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
