use super::*;
use pretty_assertions::assert_eq;

#[async_std::test]
async fn sets_and_gets_keys() -> anyhow::Result<()> {
    let service = Service::default();

    assert_eq!(
        "key does not exist",
        service.json_get("key").await.unwrap_err().to_string()
    );

    service.json_set("key", r#""value""#).await?;

    assert_eq!(r#""value""#, service.json_get("key").await?);

    Ok(())
}

#[async_std::test]
async fn replaces_existing_key() -> anyhow::Result<()> {
    let service = Service::default();

    service.json_set("key", r#""value1""#).await?;
    service.json_set("key", r#""value2""#).await?;
    service.json_set("key", r#""value3""#).await?;

    assert_eq!(r#""value3""#, service.json_get("key").await?);

    Ok(())
}

#[async_std::test]
async fn deletes_keys() -> anyhow::Result<()> {
    let service = Service::default();

    service.json_set("key", r#""value""#).await?;
    service.json_get("key").await?;

    service.json_del("key").await?;

    assert_eq!(
        "key does not exist",
        service.json_get("key").await.unwrap_err().to_string()
    );

    Ok(())
}

#[async_std::test]
async fn validates_json() -> anyhow::Result<()> {
    let service = Service::default();

    assert_eq!(
        "Execution Error: error returned from database: (code: 1) malformed JSON",
        service
            .json_set("key", "value")
            .await
            .unwrap_err()
            .to_string()
    );

    Ok(())
}

#[async_std::test]
async fn formats_json() -> anyhow::Result<()> {
    let service = Service::default();

    service.json_set("key", r#"{"key":"value"}"#).await?;

    assert_eq!(
        r#"{
    "key": "value"
}"#,
        service.json_get("key").await?
    );

    Ok(())
}

#[async_std::test]
async fn maintains_order() -> anyhow::Result<()> {
    let service = Service::default();

    service
        .json_set(
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
        service.json_get("key").await?
    );

    Ok(())
}
