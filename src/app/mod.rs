mod json;

#[derive(Debug)]
pub struct App {
    db: rusqlite::Connection,
}

impl App {
    pub fn new(db: rusqlite::Connection) -> Self {
        Self { db }
    }
}

#[cfg(test)]
impl Default for App {
    fn default() -> Self {
        let mut db = rusqlite::Connection::open_in_memory().expect("opening sqlite in memory");

        crate::migrations::run(&mut db).expect("migrating database");

        Self { db }
    }
}
