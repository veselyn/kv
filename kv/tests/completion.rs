mod common;

use assert_cmd::crate_name;
use common::*;
use predicates::prelude::*;

#[test]
fn generates_bash_completion() {
    let cmd = Cli::new().to_cmd();

    cmd()
        .args(["completion", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            format!("_{}() {{", crate_name!()).nl(),
        ));
}

#[test]
fn generates_fish_completion() {
    let cmd = Cli::new().to_cmd();

    cmd()
        .args(["completion", "fish"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.".nl()));
}

#[test]
fn generates_zsh_completion() {
    let cmd = Cli::new().to_cmd();

    cmd()
        .args(["completion", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            format!("#compdef {}", crate_name!()).nl(),
        ));
}
