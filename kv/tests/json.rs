mod common;

use common::*;

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
