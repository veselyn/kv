use super::*;
use pretty_assertions::assert_eq;
use serde_json::json;

#[async_std::test]
async fn sets_and_gets_key_without_path() -> anyhow::Result<()> {
    let service = Service::default();

    service.set("key", None, r#""value""#).await?;

    assert_eq!(json!("value"), service.get("key", None).await?);

    Ok(())
}

#[async_std::test]
async fn sets_and_gets_key_with_root_path() -> anyhow::Result<()> {
    let service = Service::default();

    service.set("key", Some("$"), r#""value""#).await?;

    assert_eq!(json!("value"), service.get("key", Some(&["$"])).await?);

    Ok(())
}

#[async_std::test]
async fn sets_and_gets_value_at_specific_path() -> anyhow::Result<()> {
    let service = Service::default();

    service.set("key", None, "{}").await?;
    service.set("key", Some("$.key"), r#""value""#).await?;

    assert_eq!(json!({"key":"value"}), service.get("key", None).await?);
    assert_eq!(json!("value"), service.get("key", Some(&["$.key"])).await?);

    Ok(())
}

#[async_std::test]
async fn fails_to_set_specific_path_of_non_existing_key() -> anyhow::Result<()> {
    let service = Service::default();

    assert!(matches!(
        service.set("key", Some("$.key"),  r#""value""#).await,
        Err(SetError::KeyNotFound(key)) if key == "key",
    ));

    Ok(())
}

#[async_std::test]
async fn sets_specific_nested_path_of_key() -> anyhow::Result<()> {
    let service = Service::default();

    service.set("key", None, "{}").await?;
    service
        .set("key", Some("$.nested.key"), r#""value""#)
        .await?;

    assert_eq!(
        json!({"nested":{"key":"value"}}),
        service.get("key", Some(&["$"])).await?
    );

    Ok(())
}

#[async_std::test]
async fn fails_to_get_non_existing_path_of_key() -> anyhow::Result<()> {
    let service = Service::default();

    service.set("key", None, "{}").await?;

    assert!(matches!(
        service.get("key", Some(&["$.key1"])).await,
        Err(GetError::PathsNotFound(paths)) if paths == ["$.key1"],
    ));

    Ok(())
}

#[async_std::test]
async fn fails_to_get_non_existing_paths_of_key() -> anyhow::Result<()> {
    let service = Service::default();

    service.set("key", None, "{}").await?;
    service.set("key", Some("$.key1"), r#""value1""#).await?;

    assert!(matches!(
        service.get("key", Some(&["$.key1", "$.key2", "$.key3"])).await,
        Err(GetError::PathsNotFound(paths)) if paths == ["$.key2", "$.key3"],
    ));

    Ok(())
}

#[async_std::test]
async fn gets_multiple_paths_of_key() -> anyhow::Result<()> {
    let service = Service::default();

    service.set("key", None, "{}").await?;
    service.set("key", Some("$.key1"), r#""value1""#).await?;
    service.set("key", Some("$.key2"), r#""value2""#).await?;
    service.set("key", Some("$.key3"), r#""value3""#).await?;

    assert_eq!(
        json!({"$.key1":"value1","$.key2":"value2","$.key3":"value3"}),
        service
            .get("key", Some(&["$.key1", "$.key2", "$.key3"]))
            .await?
    );

    Ok(())
}

#[async_std::test]
async fn gets_duplicate_path_of_key_once() -> anyhow::Result<()> {
    let service = Service::default();

    service.set("key", None, "{}").await?;
    service.set("key", Some("$.key"), r#""value""#).await?;

    assert_eq!(
        json!({"$.key":"value"}),
        service
            .get("key", Some(&["$.key", "$.key", "$.key"]))
            .await?
    );

    Ok(())
}

#[async_std::test]
async fn replaces_existing_key_without_path() -> anyhow::Result<()> {
    let service = Service::default();

    service.set("key", None, r#""value1""#).await?;
    service.set("key", None, r#""value2""#).await?;
    service.set("key", None, r#""value3""#).await?;

    assert_eq!(json!("value3"), service.get("key", None).await?);

    Ok(())
}

#[async_std::test]
async fn replaces_existing_key_with_root_path() -> anyhow::Result<()> {
    let service = Service::default();

    service.set("key", Some("$"), r#""value1""#).await?;
    service.set("key", Some("$"), r#""value2""#).await?;
    service.set("key", Some("$"), r#""value3""#).await?;

    assert_eq!(json!("value3"), service.get("key", None).await?);

    Ok(())
}

#[async_std::test]
async fn replaces_value_at_specific_path() -> anyhow::Result<()> {
    let service = Service::default();

    service.set("key", None, "{}").await?;
    service.set("key", Some("$.key"), r#""value1""#).await?;
    service.set("key", Some("$.key"), r#""value2""#).await?;
    service.set("key", Some("$.key"), r#""value3""#).await?;

    assert_eq!(json!("value3"), service.get("key", Some(&["$.key"])).await?);

    Ok(())
}

#[async_std::test]
async fn deletes_key_without_path() -> anyhow::Result<()> {
    let service = Service::default();

    service.set("key", None, r#""value""#).await?;
    service.get("key", None).await?;

    service.del("key", None).await?;

    assert!(matches!(
        service.get("key", None).await,
        Err(GetError::KeyNotFound(key)) if key == "key",
    ));

    Ok(())
}

#[async_std::test]
async fn deletes_key_with_root_path() -> anyhow::Result<()> {
    env_logger::init();

    let service = Service::default();

    service.set("key", None, r#""value""#).await?;
    service.get("key", None).await?;

    service.del("key", Some("$")).await?;

    assert!(matches!(
        service.get("key", None).await,
        Err(GetError::KeyNotFound(key)) if key == "key",
    ));

    Ok(())
}

#[async_std::test]
async fn deletes_value_at_specific_path() -> anyhow::Result<()> {
    let service = Service::default();

    service.set("key", None, "{}").await?;
    service.set("key", Some("$.key"), r#""value""#).await?;
    service.get("key", None).await?;

    service.del("key", Some("$.key")).await?;

    assert_eq!(json!({}), service.get("key", None).await?);

    Ok(())
}

#[async_std::test]
async fn fails_to_delete_non_existing_key_without_path() -> anyhow::Result<()> {
    let service = Service::default();

    assert!(matches!(
        service.del("key", None).await,
        Err(DelError::KeyNotFound(key)) if key == "key",
    ));

    Ok(())
}

#[async_std::test]
async fn fails_to_delete_non_existing_key_with_root_path() -> anyhow::Result<()> {
    let service = Service::default();

    assert!(matches!(
        service.del("key", Some("$")).await,
        Err(DelError::KeyNotFound(key)) if key == "key",
    ));

    Ok(())
}

#[async_std::test]
async fn fails_to_delete_non_existing_value_at_specific_path() -> anyhow::Result<()> {
    let service = Service::default();

    service.set("key", None, "{}").await?;

    assert!(matches!(
        service.del("key", Some("$.key")).await,
        Err(DelError::PathNotFound(path)) if path == "$.key",
    ));

    Ok(())
}

#[async_std::test]
async fn validates_json() -> anyhow::Result<()> {
    let service = Service::default();

    assert!(matches!(
        service.set("key", None, "value").await,
        Err(SetError::InvalidJson(_)),
    ));

    Ok(())
}
