use super::*;
use pretty_assertions::assert_eq;

#[async_std::test]
async fn sets_and_gets_keys() -> anyhow::Result<()> {
    let service = Service::default();

    assert_eq!(
        "key does not exist",
        service.get("key").await.unwrap_err().to_string()
    );

    service.set("key", r#""value""#).await?;

    assert_eq!(r#""value""#, service.get("key").await?);

    Ok(())
}

#[async_std::test]
async fn replaces_existing_key() -> anyhow::Result<()> {
    let service = Service::default();

    service.set("key", r#""value1""#).await?;
    service.set("key", r#""value2""#).await?;
    service.set("key", r#""value3""#).await?;

    assert_eq!(r#""value3""#, service.get("key").await?);

    Ok(())
}

#[async_std::test]
async fn deletes_keys() -> anyhow::Result<()> {
    let service = Service::default();

    service.set("key", r#""value""#).await?;
    service.get("key").await?;

    service.del("key").await?;

    assert_eq!(
        "key does not exist",
        service.get("key").await.unwrap_err().to_string()
    );

    Ok(())
}

#[async_std::test]
async fn validates_json() -> anyhow::Result<()> {
    let service = Service::default();

    assert_eq!(
        "Execution Error: error returned from database: (code: 1) malformed JSON",
        service.set("key", "value").await.unwrap_err().to_string()
    );

    Ok(())
}

#[async_std::test]
async fn formats_json() -> anyhow::Result<()> {
    let service = Service::default();

    service.set("key", r#"{"key":"value"}"#).await?;

    assert_eq!(
        r#"{
    "key": "value"
}"#,
        service.get("key").await?
    );

    Ok(())
}

#[async_std::test]
async fn maintains_order() -> anyhow::Result<()> {
    let service = Service::default();

    service
        .set(
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
        service.get("key").await?
    );

    Ok(())
}
