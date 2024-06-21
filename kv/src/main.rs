mod app;
mod cli;
mod database;

use clap::Parser;
use cli::Cli;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.run().await?;
    Ok(())
}
