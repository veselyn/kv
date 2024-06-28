use clap::Parser;
use kv::cli::Cli;
use std::process::ExitCode;

#[async_std::main]
async fn main() -> ExitCode {
    Cli::parse()
        .run()
        .await
        .inspect(|output| output.dump())
        .inspect_err(|err| err.dump())
        .map_or_else(|err| err.status.into(), |_| ExitCode::SUCCESS)
}
