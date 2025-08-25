use std::result;

use rusqlite::{Connection, OptionalExtension};
use rusqlite_migration::{M, Migrations};

const MIGRATIONS_SLICE: &[M<'_>] = &[M::up(include_str!("migrations/initial.sql"))];
const MIGRATIONS: Migrations<'_> = Migrations::from_slice(MIGRATIONS_SLICE);

pub struct Database {
    connection: Connection,
}
impl Database {
    pub fn new(connection: Connection) -> Self {
        // todo: is it good idea to use migrations here?
        let mut db = Self { connection };
        db.apply_migrations();
        db
    }

    fn apply_migrations(&mut self) {
        self.connection
            .pragma_update_and_check(None, "journal_mode", "WAL", |_| Ok(()))
            .unwrap();

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
            let result = tx
                .prepare(
                    "SELECT * FROM tags 
                WHERE name = ?1",
                )?
                .exists([&tag])?;
            if !result {
                let mut insert = tx.prepare(
                    "INSERT INTO tags (name) 
                VALUES (?1) 
                RETURNING id",
                )?;
                let tag = insert.query_one([tag.clone()], |row| {
                    Ok(Tag {
                        id: row.get(0)?,
                        name: tag,
                    })
                })?;
                db_tags.push(tag);
            }
        }
        {
            let mut select = tx.prepare(
                "SELECT id FROM files 
            WHERE hash_sum = ?1",
            )?;
            let result: Option<i32> = select.query_one([&hash_sum], |row| row.get(0)).optional()?;
            let file_id = match result {
                Some(file_id) => file_id,
                None => {
                    let mut insert = tx.prepare(
                        "INSERT INTO files (path, name, hash_sum) 
                        VALUES (?1, ?2, ?3) 
                        RETURNING id",
                    )?;
                    insert.query_one((file_path, file_name, hash_sum), |row| row.get(0))?
                }
            };

            for tag in db_tags {
                tx.execute(
                    "INSERT INTO file_tags (file_id, tag_id) VALUES (?1, ?2)",
                    (file_id, tag.id),
                )?;
            }
        }

        tx.commit()?;

        Ok(())
    }

    pub fn get_tags(&self, hash_sum: &str) -> anyhow::Result<Vec<String>> {
        let mut select = self.connection.prepare(
            "SELECT t.name 
            FROM tags t 
            INNER JOIN file_tags ON file_tags.tag_id = t.id 
            INNER JOIN files ON files.id = file_tags.file_id AND files.hash_sum = ?1",
        )?;
        let result = select.query_map([&hash_sum], |row| row.get(0))?;
        let mut tags: Vec<String> = Vec::new();
        for tag in result {
            tags.push(tag?);
        }
        Ok(tags)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Tag {
    id: i32,
    name: String,
}

#[derive(Debug, PartialEq, Eq)]
struct File {
    id: i32,
    path: String,
    name: String,
    hash_sum: String,
}
