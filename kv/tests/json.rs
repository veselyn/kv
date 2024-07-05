mod common;

use common::*;
use std::io::Write;
use std::process::{self, Stdio};

#[test]
fn gets_sets_and_deletes_key() {
    let cmd = Cli::new().to_cmd();

    cmd()
        .args(["json", "get", "key"])
        .assert()
        .failure()
        .stdout("")
        .stderr(r#"Error: key "key" not found"#.nl());

    cmd()
        .args(["json", "set", "key", r#""value""#])
        .assert()
        .success()
        .stdout("")
        .stderr("");

    cmd()
        .args(["json", "get", "key"])
        .assert()
        .success()
        .stdout(r#""value""#.nl())
        .stderr("");

    cmd()
        .args(["json", "del", "key"])
        .assert()
        .success()
        .stdout("")
        .stderr("");

    cmd()
        .args(["json", "get", "key"])
        .assert()
        .failure()
        .stdout("")
        .stderr(r#"Error: key "key" not found"#.nl());
}

#[test]
fn replicates_jq_output() {
    let value = r#"{"key":"value"}"#;

    let mut jq_child = process::Command::new("jq")
        .args(["--indent", "4"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    jq_child
        .stdin
        .take()
        .unwrap()
        .write_all(value.as_bytes())
        .unwrap();

    let jq_output = jq_child.wait_with_output().unwrap();
    assert!(jq_output.status.success());

    let cmd = Cli::new().to_cmd();

    cmd()
        .args(["json", "set", "key", value])
        .assert()
        .success()
        .stdout("")
        .stderr("");

    cmd()
        .args(["json", "get", "key"])
        .assert()
        .success()
        .stdout(String::from_utf8(jq_output.stdout).unwrap())
        .stderr("");
}
