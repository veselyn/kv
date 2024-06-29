use clap::Parser;
use kv::{app::App, config::Config, Cli};
use pretty_assertions::assert_eq;
use tempfile::NamedTempFile;

#[async_std::test]
async fn formats_output() -> anyhow::Result<()> {
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

    assert_eq!("", result.stderr);
    assert_eq!(
        r#"{
    "key": "value"
}"#,
        result.stdout
    );

    Ok(())
}
