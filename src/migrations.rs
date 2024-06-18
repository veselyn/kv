use migration::MigratorTrait;

pub async fn run<'c, C>(database: C) -> anyhow::Result<()>
where
    C: migration::IntoSchemaManagerConnection<'c>,
{
    migration::Migrator::up(database, None).await?;
    Ok(())
}
