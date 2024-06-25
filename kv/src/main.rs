mod app;
mod cli;
mod database;
mod json;

use clap::Parser;
use cli::Cli;

#[async_std::main]
async fn main() -> cli::Result {
    let cli = Cli::parse();

    let result = cli.run().await;
    result.dump();
    result
}
