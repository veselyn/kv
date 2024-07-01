use async_std::sync::{Arc, Mutex};
use clap::Parser;
use kv::env::stdio::{MemoryStderr, MemoryStdout};
use kv::env::Env;
use kv::{app::App, config::Config, Cli};
use pretty_assertions::assert_eq;
use std::io::Write;
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

#[async_std::test]
async fn formats_output_with_jq() -> anyhow::Result<()> {
    let temp_file = NamedTempFile::new()?;
    let db_path = temp_file.path();
    let stdout = Arc::new(Mutex::new(MemoryStdout::new(false)));
    let stderr = Arc::new(Mutex::new(MemoryStderr::new(false)));

    let env = Env::builder()
        .stdout(stdout.clone())
        .stderr(stderr.clone())
        .build();

    let config = Config::builder().db_path(db_path.to_path_buf()).build()?;

    let app = App::builder().env(env).config(config).build().await?;

    Cli::parse_from(["", "json", "set", "key", r#"{"key":"value"}"#])
        .run_with(&app)
        .await?;

    let result = Cli::parse_from(["", "json", "get", "key"])
        .run_with(&app)
        .await?;

    result.dump().await;

    let mut jq = Command::new("jq")
        .env("JQ_COLORS", "0;90:0;39:0;39:0;39:0;32:1;39:1;39:34;1")
        .arg("--color-output")
        .args(["--indent", "4"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    jq.stdin
        .as_mut()
        .unwrap()
        .write_all(br#"{"key":"value"}"#)?;

    let output = jq.wait_with_output()?;
    assert!(output.status.success());

    assert_eq!("", String::from_utf8(stderr.lock().await.buf.clone())?);
    std::assert_eq!(
        String::from_utf8(output.stdout)?,
        String::from_utf8(stdout.lock().await.buf.clone())?
    );

    Ok(())
}
