use clap::Parser;
use kv::Cli;
use std::process::ExitCode;

#[async_std::main]
async fn main() -> ExitCode {
    env_logger::init();

    match Cli::parse().run().await {
        Ok(output) => {
            output.dump().await;
            ExitCode::SUCCESS
        }
        Err(err) => {
            err.dump().await;
            ExitCode::from(err.status)
        }
    }
}
