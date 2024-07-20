use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(about = "Test")]
    Test,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Test => test(&cli),
    }
}

fn test(_: &Cli) -> Result<()> {
    Ok(())
}
