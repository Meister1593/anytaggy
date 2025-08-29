mod tables;

use crate::db::tables::{
    file_tags::{
        get_file_paths_by_tags_and_op, get_file_tag_ids_by_id, get_file_tags_by_hash,
        reference_file_tag, unreference_file_tag,
    },
    files::{create_file, delete_file, get_all_files_path, get_file_id},
    tags::{create_tag, get_tag_by_name, get_tag_id_by_name, get_tag_names},
};
use anyhow::{Result, bail};
use rusqlite::{Connection, OpenFlags};
use rusqlite_migration::{M, Migrations};
use std::{path::Path, vec};
use tracing::{debug, info};

const MIGRATIONS_SLICE: &[M] = &[M::up(include_str!("migrations/initial.sql"))];
const MIGRATIONS: Migrations = Migrations::from_slice(MIGRATIONS_SLICE);

#[derive(Debug, Clone)]
pub struct File<'a> {
    pub path: &'a str,
    pub name: &'a str,
    pub contents_hash: &'a str,
    pub fingerprint_hash: &'a str,
}

pub enum DatabaseMode {
    ReadWrite,
    Read,
}
pub struct Database {
    connection: Connection,
}
impl Database {
    fn apply_migrations(&mut self) {
        self.connection
            .pragma_update_and_check(None, "journal_mode", "WAL", |_| Ok(()))
            .unwrap();

        MIGRATIONS.to_latest(&mut self.connection).unwrap();
    }

    // todo: the only place where unwrap is used, is it fine?
    pub fn new(database_mode: &DatabaseMode, database_path: &Path) -> Self {
        let connection = match database_mode {
            DatabaseMode::ReadWrite => Connection::open(database_path).unwrap(),

            DatabaseMode::Read => Connection::open_with_flags(
                database_path,
                OpenFlags::SQLITE_OPEN_READ_ONLY
                    | OpenFlags::SQLITE_OPEN_NO_MUTEX
                    | OpenFlags::SQLITE_OPEN_URI,
            )
            .unwrap(),
        };
        match database_mode {
            DatabaseMode::ReadWrite => {
                // todo: is it good idea to use migrations here?
                let mut db = Self { connection };
                db.apply_migrations();
                db
            }
            DatabaseMode::Read => Self { connection },
        }
    }

    pub fn tag_file(&mut self, file: &File, tag_names: &[String]) -> Result<()> {
        let tx = self.connection.transaction()?;
        let mut db_tag_ids = vec![];

        for tag_name in tag_names {
            let tag_name = tag_name.trim();
            let tag_id = if let Some(tag_id) = get_tag_id_by_name(&tx, tag_name)? {
                tag_id
            } else {
                let db_tag = create_tag(&tx, tag_name)?;
                info!("created tag: {tag_name}");
                db_tag.id
            };
            debug!("tag_id: {tag_id}");
            db_tag_ids.push(tag_id);
        }

        // todo: this looks kinda ugly, might be better to use unwrap_or_else (but then no automatic ?)
        let file_id = if let Some(file_id) = get_file_id(&tx, file.fingerprint_hash)? {
            debug!("found file_id {file_id}");
            file_id
        } else {
            let db_file = create_file(&tx, file)?;
            db_file.id
        };
        debug!("file_id: {file_id}");

        let file_tag_ids = get_file_tag_ids_by_id(&tx, file_id)?;
        for tag_id in db_tag_ids {
            if !file_tag_ids.contains(&tag_id) {
                reference_file_tag(&tx, file_id, tag_id)?;
            }
        }

        tx.commit()?;

        Ok(())
    }

    pub fn get_file_tags(&self, fingerprint_hash: &str) -> Result<Vec<String>> {
        get_file_tags_by_hash(&self.connection, fingerprint_hash)
    }

    pub fn get_files_by_tag(&self, tag_names: &[String]) -> Result<Vec<String>> {
        get_file_paths_by_tags_and_op(&self.connection, tag_names)
    }

    pub fn get_all_tags(&self) -> Result<Vec<String>> {
        get_tag_names(&self.connection)
    }

    pub fn delete_tags_from_file(&mut self, file: &File, tag_names: &[String]) -> Result<()> {
        let tx = self.connection.transaction()?;
        let mut db_tag_ids = vec![];

        for tag_name in tag_names {
            let Some(tag) = get_tag_by_name(&tx, tag_name)? else {
                bail!("Could not find such tag in database: {tag_name}");
            };
            debug!("found tag_id {}", tag.id);

            db_tag_ids.push(tag);
        }
        let Some(file_id) = get_file_id(&tx, file.fingerprint_hash)? else {
            bail!("Could not find such file in database");
        };
        debug!("found file_id {file_id}");

        let file_tag_ids = get_file_tag_ids_by_id(&tx, file_id)?;
        for tag in &db_tag_ids {
            if file_tag_ids.contains(&tag.id) {
                unreference_file_tag(&tx, file_id, tag.id)?;
            } else {
                bail!("File did not have such tag: {}", tag.name);
            }
        }

        if file_tag_ids.len() == db_tag_ids.len() {
            delete_file(&tx, file_id)?;
        }

        tx.commit()?;

        Ok(())
    }

    pub(crate) fn get_files(&self) -> Result<Vec<String>> {
        get_all_files_path(&self.connection)
    }
}
