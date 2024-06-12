use super::*;
use pretty_assertions::assert_eq;

#[test]
fn sets_and_gets_keys() -> anyhow::Result<()> {
    let app = App::default();

    assert_eq!(
        rusqlite::Error::QueryReturnedNoRows,
        app.json_get("key").unwrap_err().downcast()?
    );

    app.json_set("key", r#""value""#)?;

    assert_eq!(r#""value""#, app.json_get("key")?);

    Ok(())
}

#[test]
fn replaces_existing_key() -> anyhow::Result<()> {
    let app = App::default();

    app.json_set("key", r#""value1""#)?;
    app.json_set("key", r#""value2""#)?;
    app.json_set("key", r#""value3""#)?;

    assert_eq!(r#""value3""#, app.json_get("key")?);

    Ok(())
}

#[test]
fn deletes_keys() -> anyhow::Result<()> {
    let app = App::default();

    app.json_set("key", r#""value""#)?;
    app.json_get("key")?;

    app.json_del("key")?;

    assert_eq!(
        rusqlite::Error::QueryReturnedNoRows,
        app.json_get("key").unwrap_err().downcast()?
    );

    Ok(())
}

#[test]
fn validates_json() -> anyhow::Result<()> {
    let app = App::default();

    assert_eq!(
        rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error {
                code: rusqlite::ErrorCode::Unknown,
                extended_code: 1,
            },
            Some("malformed JSON".to_string())
        ),
        app.json_set("key", "value").unwrap_err().downcast()?
    );

    Ok(())
}

#[test]
fn formats_json() -> anyhow::Result<()> {
    let app = App::default();

    app.json_set("key", r#"{"key":"value"}"#)?;

    assert_eq!(
        r#"{
    "key": "value"
}"#,
        app.json_get("key")?
    );

    Ok(())
}
