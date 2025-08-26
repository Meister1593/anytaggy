mod tables;

use anyhow::Result;
use rusqlite::Connection;
use rusqlite_migration::{M, Migrations};

use crate::db::tables::{
    file_tags::{get_file_tags, reference_file_tag},
    files::{create_file, get_file_id},
    tags::{create_tag, does_tag_exist_by_name},
};

const MIGRATIONS_SLICE: &[M] = &[M::up(include_str!("migrations/initial.sql"))];
const MIGRATIONS: Migrations = Migrations::from_slice(MIGRATIONS_SLICE);

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

    pub fn tag(
        &mut self,
        file_path: &str,
        file_name: &str,
        hash_sum: &str,
        tags: Vec<String>,
    ) -> Result<()> {
        let tx = self.connection.transaction()?;
        let mut db_tags = vec![];
        for tag in tags {
            let tag = tag.trim();
            if !does_tag_exist_by_name(&tx, tag)? {
                db_tags.push(create_tag(&tx, tag)?);
            }
        }

        let file_id_option = get_file_id(&tx, hash_sum)?;
        // todo: this looks kinda ugly, might be better to use unwrap_or_else (but then no automatic ?)
        let file_id = match file_id_option {
            Some(file_id) => file_id,
            None => create_file(&tx, file_path, file_name, hash_sum)?,
        };

        for tag_id in db_tags {
            reference_file_tag(&tx, file_id, tag_id)?;
        }

        tx.commit()?;

        Ok(())
    }

    pub fn get_tags(&self, hash_sum: &str) -> Result<Vec<String>> {
        get_file_tags(&self.connection, hash_sum)
    }
}
