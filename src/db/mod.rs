use std::{fs::File, io::Read, path::PathBuf};

use anyhow::Context;
use clap::builder::Str;
use rusqlite::Connection;
use rusqlite_migration::{M, Migrations};
use sha_rs::Sha;

use crate::commands::{tag, tags};

const MIGRATIONS_SLICE: &[M<'_>] = &[M::up(include_str!("migrations/initial.sql"))];
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

    pub fn tag(
        &mut self,
        file_path: &str,
        file_name: &str,
        hash_sum: &str,
        tags: Vec<String>,
    ) -> anyhow::Result<()> {
        let tx = self.connection.transaction()?;
        let mut db_tags: Vec<Tag> = vec![];
        for tag in tags {
            let mut result = tx
                .prepare("SELECT COUNT(1) FROM tags WHERE name = ?1")?
                .query([tag])?;
            let row = result.next()?.unwrap();
            let r: u32 = row.get(0)?;
            if r == 0 {
                let mut insert = tx.prepare("INSERT INTO tags (name) VALUES (?1) RETURNING id")?;
                let tag = insert.query_one([tag.clone()], |row| {
                    Ok(Tag {
                        id: row.get(0)?,
                        name: tag,
                    })
                })?;
                db_tags.push(tag);
            }
        }
        tx.execute(
            "INSERT INTO files (path, name, hash_sum) VALUES (?1, ?2, ?3)",
            (file_path, file_name, hash_sum),
        )?;

        tx.commit()?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Tag {
    id: i32,
    name: String,
}
