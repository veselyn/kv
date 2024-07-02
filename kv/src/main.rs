use clap::Parser;
use kv::Cli;
use std::process::ExitCode;

#[async_std::main]
async fn main() -> ExitCode {
    env_logger::init();

    match Cli::parse().run().await {
        Ok(mut output) => {
            output.dump();
            ExitCode::SUCCESS
        }
        Err(err) => {
            err.dump();
            err.status.into()
        }
    }
}
