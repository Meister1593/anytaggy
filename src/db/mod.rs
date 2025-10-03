mod tables;

use rusqlite::{Connection, OpenFlags};
use rusqlite_migration::{M, Migrations};
use std::path::Path;

const MIGRATIONS_SLICE: &[M] = &[M::up(include_str!("migrations/initial.sql"))];
const MIGRATIONS: Migrations = Migrations::from_slice(MIGRATIONS_SLICE);

#[derive(Debug, Clone)]
pub struct File {
    pub path: String,
    pub name: String,
    pub contents_hash: String,
    pub fingerprint_hash: String,
}

pub enum DatabaseMode {
    ReadWriteCreate,
    ReadWrite,
    Read,
}
pub struct Database {
    connection: Connection,
}
impl Database {
    fn apply_migrations(&mut self) {
        MIGRATIONS.to_latest(&mut self.connection).unwrap();
    }

    // todo: the only place where unwrap is used, is it fine?
    pub fn new(database_mode: &DatabaseMode, database_path: &Path) -> Self {
        let connection = match database_mode {
            DatabaseMode::ReadWriteCreate => Connection::open(database_path).unwrap(),
            DatabaseMode::ReadWrite => Connection::open_with_flags(
                database_path,
                OpenFlags::SQLITE_OPEN_READ_WRITE
                    | OpenFlags::SQLITE_OPEN_NO_MUTEX
                    | OpenFlags::SQLITE_OPEN_URI,
            )
            .unwrap(),
            DatabaseMode::Read => Connection::open_with_flags(
                database_path,
                OpenFlags::SQLITE_OPEN_READ_ONLY
                    | OpenFlags::SQLITE_OPEN_NO_MUTEX
                    | OpenFlags::SQLITE_OPEN_URI,
            )
            .unwrap(),
        };
        match database_mode {
            DatabaseMode::ReadWrite | DatabaseMode::ReadWriteCreate => {
                // todo: is it good idea to use migrations here?
                let mut db: Database = Self { connection };
                db.apply_migrations();
                db
            }
            DatabaseMode::Read => Self { connection },
        }
    }
}
