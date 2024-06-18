mod json;

#[derive(Debug)]
pub struct App {
    db: sea_orm::DatabaseConnection,
}

impl App {
    pub fn new(db: sea_orm::DatabaseConnection) -> Self {
        Self { db }
    }
}

#[cfg(test)]
impl Default for App {
    fn default() -> Self {
        let db = async_std::task::block_on(async {
            let db = crate::database::new("sqlite::memory:")
                .await
                .expect("opening sqlite in memory");

            crate::migrations::run(&db)
                .await
                .expect("migrating database");

            db
        });

        Self { db }
    }
}
