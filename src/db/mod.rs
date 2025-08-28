mod tables;

use crate::db::tables::{
    file_tags::{
        get_file_paths_by_tags_and_op, get_file_tag_ids_by_id, get_file_tags_by_hash,
        reference_file_tag,
    },
    files::{create_file, get_file_id},
    tags::{create_tag, get_tag_id_by_name, get_tags},
};
use anyhow::Result;
use rusqlite::Connection;
use rusqlite_migration::{M, Migrations};
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

    pub fn new(connection: Connection) -> Self {
        // todo: is it good idea to use migrations here?
        let mut db = Self { connection };
        db.apply_migrations();
        db
    }

    pub fn tag_file(&mut self, file: &File, tag_names: Vec<String>) -> Result<()> {
        let tx = self.connection.transaction()?;
        let mut db_tags = vec![];

        for tag_name in tag_names {
            let tag_name = tag_name.trim();
            let tag_id = if let Some(tag_id) = get_tag_id_by_name(&tx, tag_name)? {
                tag_id
            } else {
                let tag_id = create_tag(&tx, tag_name)?;
                info!("created tag: {tag_name}");
                tag_id
            };
            debug!("tag_id: {tag_id}");
            db_tags.push(tag_id);
        }

        // todo: this looks kinda ugly, might be better to use unwrap_or_else (but then no automatic ?)
        let file_id = if let Some(file_id) = get_file_id(&tx, file.fingerprint_hash)? {
            debug!("found file_id {file_id}");
            file_id
        } else {
            create_file(&tx, file)?
        };
        debug!("file_id: {file_id}");

        let file_tag_ids = get_file_tag_ids_by_id(&tx, file_id)?;
        for tag_id in db_tags {
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

    pub fn get_files_by_tag(&self, tag_names: Vec<String>) -> Result<Vec<String>> {
        get_file_paths_by_tags_and_op(&self.connection, tag_names)
    }

    pub fn get_all_tags(&self) -> Result<Vec<String>> {
        get_tags(&self.connection)
    }
}
