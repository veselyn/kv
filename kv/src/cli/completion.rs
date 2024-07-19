use super::command::{self, Execute};
use super::Cli;
use crate::app::App;
use clap::CommandFactory;
use clap::Subcommand;
use clap_complete::{generate, shells};
use std::io::Cursor;

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    #[command(about = "Generate completion script for Bash")]
    Bash,
    #[command(about = "Generate completion script for Fish")]
    Fish,
    #[command(about = "Generate completion script for Zsh")]
    Zsh,
}

impl Execute for Command {
    async fn execute(self, _: &App) -> command::Result {
        let mut cmd = Cli::command();
        let bin_name = env!("CARGO_PKG_NAME");
        let mut buf = Vec::new();

        match self {
            Command::Bash => generate(shells::Bash, &mut cmd, bin_name, &mut buf),
            Command::Fish => generate(shells::Fish, &mut cmd, bin_name, &mut buf),
            Command::Zsh => generate(shells::Zsh, &mut cmd, bin_name, &mut buf),
        };

        let trimmed = String::from_utf8(buf)
            .expect("completion script is not utf8")
            .trim()
            .to_owned();

        Ok(command::Output::default().stdout(Cursor::new(trimmed)))
    }
}
