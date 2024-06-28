use super::*;
use pretty_assertions::assert_eq;

#[async_std::test]
async fn sets_and_gets_keys() -> anyhow::Result<()> {
    let service = Service::default();

    assert!(matches!(
        service.get("key").await,
        Err(GetError::KeyNotFound(key)) if key == "key",
    ));

    service.set("key", r#""value""#).await?;

    assert_eq!(r#""value""#, service.get("key").await?);

    Ok(())
}

#[async_std::test]
async fn replaces_existing_keys() -> anyhow::Result<()> {
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

    assert!(matches!(
        service.get("key").await,
        Err(GetError::KeyNotFound(key)) if key == "key",
    ));

    Ok(())
}

#[async_std::test]
async fn fails_to_delete_non_existing_keys() -> anyhow::Result<()> {
    let service = Service::default();

    assert!(matches!(
        service.del("key").await,
        Err(DelError::KeyNotFound(key)) if key == "key",
    ));

    Ok(())
}

#[async_std::test]
async fn validates_json() -> anyhow::Result<()> {
    let service = Service::default();

    assert!(matches!(
        service.set("key", "value").await,
        Err(SetError::InvalidJson(_)),
    ));

    Ok(())
}

#[async_std::test]
async fn minifies_json() -> anyhow::Result<()> {
    let service = Service::default();

    service.set("key", r#" {  "key"   : "value"  }   "#).await?;

    assert_eq!(r#"{"key":"value"}"#, service.get("key").await?);

    Ok(())
}
