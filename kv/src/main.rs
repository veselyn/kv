mod app;
mod cli;
mod database;
mod json;

use clap::Parser;
use cli::Cli;
use std::process::ExitCode;

#[async_std::main]
async fn main() -> ExitCode {
    Cli::parse()
        .run()
        .await
        .inspect(|output| output.dump())
        .inspect_err(|err| err.dump())
        .map_or_else(|err| err.status, |_| ExitCode::SUCCESS)
}
