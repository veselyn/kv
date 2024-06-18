pub async fn new<C>(options: C) -> anyhow::Result<sea_orm::DatabaseConnection>
where
    C: Into<sea_orm::ConnectOptions>,
{
    Ok(sea_orm::Database::connect(options).await?)
}
