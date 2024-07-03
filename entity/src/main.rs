#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let temp_file = tempfile::NamedTempFile::new()?;
    let temp_file_path = temp_file.path();
    let database_url = format!(
        "sqlite://{}",
        temp_file_path
            .to_owned()
            .into_os_string()
            .into_string()
            .map_err(|err| anyhow::anyhow!(
                "invalid temp file path: {}",
                err.to_string_lossy().to_string()
            ))?
    );

    sea_orm_cli::run_migrate_command(
        Some(sea_orm_cli::MigrateSubcommands::Fresh),
        "./migration",
        None,
        Some(database_url.clone()),
        false,
    )
    .map_err(|err| anyhow::anyhow!(err.to_string()))?;

    sea_orm_cli::run_generate_command(
        sea_orm_cli::GenerateSubcommands::Entity {
            compact_format: false,
            expanded_format: true,
            include_hidden_tables: false,
            tables: vec![],
            ignore_tables: vec!["seaql_migrations".to_owned()],
            max_connections: 1,
            output_dir: "./entity/src".to_owned(),
            database_schema: "".to_owned(),
            database_url,
            with_serde: "none".to_owned(),
            serde_skip_deserializing_primary_key: false,
            serde_skip_hidden_column: false,
            with_copy_enums: false,
            date_time_crate: sea_orm_cli::DateTimeCrate::Chrono,
            lib: true,
            model_extra_derives: vec![],
            model_extra_attributes: vec![],
            enum_extra_derives: vec![],
            enum_extra_attributes: vec![],
            seaography: false,
        },
        false,
    )
    .await
    .map_err(|err| anyhow::anyhow!(err.to_string()))?;

    Ok(())
}
