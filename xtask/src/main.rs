use anyhow::Result;
use clap::{Args, CommandFactory, Parser, Subcommand};
use clap_mangen::Man;
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(about = "Build the man page")]
    BuildManPage(BuildManPageArgs),
}

#[derive(Args, Debug)]
struct BuildManPageArgs {
    #[arg(
        short,
        long,
        help = "Path for the output directory",
        value_name = "DIRECTORY"
    )]
    output: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::BuildManPage(args) => build_man_page(args),
    }
}

fn build_man_page(args: BuildManPageArgs) -> Result<()> {
    fn recurse(directory: &Path, cmd: clap::Command) -> Result<()> {
        let man = Man::new(cmd.clone());

        let filepath = directory.join(man.get_filename());
        let mut output = File::options()
            .create_new(true)
            .write(true)
            .open(filepath)?;

        man.render(&mut output)?;

        for subcmd in cmd.get_subcommands() {
            recurse(
                directory,
                subcmd
                    .to_owned()
                    .name(format!("{}-{}", cmd.get_name(), subcmd.get_name())),
            )?;
        }

        Ok(())
    }

    let directory = PathBuf::from(args.output);
    let cmd = kv::Cli::command();

    recurse(&directory, cmd)?;

    Ok(())
}
