use rusqlite::Connection;
use rusqlite_migration::{M, Migrations};

// 1️⃣ Define migrations
const MIGRATIONS_SLICE: &[M<'_>] = &[
    M::up(include_str!("migrations/initial.sql")),
    // In the future, add more migrations here:
    //M::up("ALTER TABLE friend ADD COLUMN email TEXT;"),
];
const MIGRATIONS: Migrations<'_> = Migrations::from_slice(MIGRATIONS_SLICE);

pub struct Database {
    connection: Connection,
}
impl Database {
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }

    pub fn apply_migrations(&mut self) {
        // Apply some PRAGMA, often better to do it outside of migrations
        self.connection
            .pragma_update_and_check(None, "journal_mode", "WAL", |_| Ok(()))
            .unwrap();

        // 2️⃣ Update the database schema, atomically
        MIGRATIONS.to_latest(&mut self.connection).unwrap();
    }
}
