mod tables;

use rusqlite::{Connection, OpenFlags};
use rusqlite_migration::{M, Migrations};
use std::path::Path;
use thiserror::Error;

const MIGRATIONS_SLICE: &[M] = &[M::up(include_str!("migrations/initial.sql"))];
const MIGRATIONS: Migrations = Migrations::from_slice(MIGRATIONS_SLICE);

#[derive(Debug, Clone)]
pub struct File {
    pub path: String,
    pub name: String,
    pub contents_hash: String,
    pub fingerprint_hash: String,
}

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Could not find such file in database")]
    NoSuchFile,
    #[error("Could not find such tag in database: {0}")]
    NoSuchTag(String),
    #[error("File did not have such tag: {0}")]
    NoSuchTagOnFile(String),
    #[error("Unhandled database error: {0}")]
    DatabaseInternal(#[from] rusqlite::Error),
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

    fn apply_runtime_options(&mut self) {
        self.connection
            .execute("PRAGMA foreign_keys = ON", [])
            .unwrap();
    }

    // todo: the only place where unwrap is used, is it fine?
    pub fn new(database_mode: &DatabaseMode, database_path: &Path) -> Result<Self, DatabaseError> {
        let connection = match database_mode {
            DatabaseMode::ReadWriteCreate => Connection::open(database_path)?,
            DatabaseMode::ReadWrite => Connection::open_with_flags(
                database_path,
                OpenFlags::SQLITE_OPEN_READ_WRITE
                    | OpenFlags::SQLITE_OPEN_NO_MUTEX
                    | OpenFlags::SQLITE_OPEN_URI,
            )?,
            DatabaseMode::Read => Connection::open_with_flags(
                database_path,
                OpenFlags::SQLITE_OPEN_READ_ONLY
                    | OpenFlags::SQLITE_OPEN_NO_MUTEX
                    | OpenFlags::SQLITE_OPEN_URI,
            )?,
        };
        match database_mode {
            DatabaseMode::ReadWrite | DatabaseMode::ReadWriteCreate => {
                // todo: is it good idea to use migrations here?
                let mut db = Self { connection };
                db.apply_runtime_options();
                db.apply_migrations();
                Ok(db)
            }
            DatabaseMode::Read => Ok(Self { connection }),
        }
    }
}
