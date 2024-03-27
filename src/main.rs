use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(about = "Get the value of a key")]
    Get { key: String },
    #[command(about = "Set the value of a key")]
    Set { key: String, value: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Get { key } => println!("getting {}", key),
        Command::Set { key, value } => println!("setting {} to {}", key, value),
    }
}
