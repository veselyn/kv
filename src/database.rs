use sea_orm::*;

pub async fn new<C>(options: C) -> anyhow::Result<DatabaseConnection>
where
    C: Into<ConnectOptions>,
{
    Ok(Database::connect(options).await?)
}
