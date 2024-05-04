pub type Migratable = rusqlite::Connection;

pub fn run(connection: &mut Migratable) -> anyhow::Result<()> {
    refinery::migrations::runner().run(connection)?;
    Ok(())
}

mod refinery {
    refinery::embed_migrations!("./migrations");
}
