mod app;
mod cli;
mod migrations;

use clap::Parser;
use cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.run()?;
    Ok(())
}
