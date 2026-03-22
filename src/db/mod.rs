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
    pub size: String,
}

#[allow(unused)]
#[derive(Default, Debug)]
pub struct DbFile {
    pub id: i32,
    pub path: String,
    pub name: String,
    pub contents_hash: String,
    pub fingerprint_hash: String,
    pub size: String,
}

impl DbFile {
    pub fn builder() -> DbFileBuilder {
        DbFileBuilder::default()
    }
}

#[derive(Default)]
pub struct DbFileBuilder {
    id: i32,
    path: String,
    name: String,
    contents_hash: String,
    fingerprint_hash: String,
    size: String,
}
impl DbFileBuilder {
    pub fn new() -> DbFileBuilder {
        DbFileBuilder {
            id: -1,
            path: String::new(),
            name: String::new(),
            contents_hash: String::new(),
            fingerprint_hash: String::new(),
            size: String::new(),
        }
    }

    pub fn id(mut self, id: i32) -> DbFileBuilder {
        self.id = id;
        self
    }

    pub fn path(mut self, path: String) -> DbFileBuilder {
        self.path = path;
        self
    }

    pub fn name(mut self, name: String) -> DbFileBuilder {
        self.name = name;
        self
    }

    pub fn contents_hash(mut self, contents_hash: String) -> DbFileBuilder {
        self.contents_hash = contents_hash;
        self
    }

    pub fn fingerprint_hash(mut self, fingerprint_hash: String) -> DbFileBuilder {
        self.fingerprint_hash = fingerprint_hash;
        self
    }

    pub fn size(mut self, size: String) -> DbFileBuilder {
        self.size = size;
        self
    }

    pub fn build(self) -> DbFile {
        DbFile {
            id: self.id,
            path: self.path,
            name: self.name,
            contents_hash: self.contents_hash,
            fingerprint_hash: self.fingerprint_hash,
            size: self.size,
        }
    }
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
