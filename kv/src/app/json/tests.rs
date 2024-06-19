use super::*;
use pretty_assertions::assert_eq;

#[async_std::test]
async fn sets_and_gets_keys() -> anyhow::Result<()> {
    let app = App::default();

    assert_eq!(
        "key does not exist",
        app.json_get("key").await.unwrap_err().to_string()
    );

    app.json_set("key", r#""value""#).await?;

    assert_eq!(r#""value""#, app.json_get("key").await?);

    Ok(())
}

#[async_std::test]
async fn replaces_existing_key() -> anyhow::Result<()> {
    let app = App::default();

    app.json_set("key", r#""value1""#).await?;
    app.json_set("key", r#""value2""#).await?;
    app.json_set("key", r#""value3""#).await?;

    assert_eq!(r#""value3""#, app.json_get("key").await?);

    Ok(())
}

#[async_std::test]
async fn deletes_keys() -> anyhow::Result<()> {
    let app = App::default();

    app.json_set("key", r#""value""#).await?;
    app.json_get("key").await?;

    app.json_del("key").await?;

    assert_eq!(
        "key does not exist",
        app.json_get("key").await.unwrap_err().to_string()
    );

    Ok(())
}

#[async_std::test]
async fn validates_json() -> anyhow::Result<()> {
    let app = App::default();

    assert_eq!(
        "Execution Error: error returned from database: (code: 1) malformed JSON",
        app.json_set("key", "value").await.unwrap_err().to_string()
    );

    Ok(())
}

#[async_std::test]
async fn formats_json() -> anyhow::Result<()> {
    let app = App::default();

    app.json_set("key", r#"{"key":"value"}"#).await?;

    assert_eq!(
        r#"{
    "key": "value"
}"#,
        app.json_get("key").await?
    );

    Ok(())
}

#[async_std::test]
async fn maintains_order() -> anyhow::Result<()> {
    let app = App::default();

    app.json_set(
        "key",
        r#"{"z_key":"value","A_key":"value","a_key":"value"}"#,
    )
    .await?;

    assert_eq!(
        r#"{
    "z_key": "value",
    "A_key": "value",
    "a_key": "value"
}"#,
        app.json_get("key").await?
    );

    Ok(())
}
