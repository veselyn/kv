use clap::Parser;
use kv::{app::App, config::Config, Cli};
use pretty_assertions::assert_eq;
use std::io::Write;
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

#[async_std::test]
async fn formats_output_with_jq() -> anyhow::Result<()> {
    let temp_file = NamedTempFile::new()?;
    let db_path = temp_file.path();

    let app = App::builder()
        .config(Config::builder().db_path(db_path.to_path_buf()).build()?)
        .build()
        .await?;

    Cli::parse_from(["", "json", "set", "key", r#"{"key":"value"}"#])
        .run_with(&app)
        .await?;

    let result = Cli::parse_from(["", "json", "get", "key"])
        .run_with(&app)
        .await?;

    let mut jq = Command::new("jq")
        .arg("--join-output")
        .arg("--color-output")
        .args(["--indent", "4"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("JQ_COLORS", "0;90:0;39:0;39:0;39:0;32:1;39:1;39:34;1")
        .spawn()?;

    jq.stdin
        .as_mut()
        .unwrap()
        .write_all(br#"{"key":"value"}"#)?;

    let output = jq.wait_with_output()?;
    assert!(output.status.success());

    assert_eq!("", result.stderr);
    std::assert_eq!(String::from_utf8(output.stdout)?, result.stdout);

    Ok(())
}
