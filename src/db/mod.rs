use anyhow::Result;
use rusqlite::{Connection, OptionalExtension, Transaction};
use rusqlite_migration::{M, Migrations};

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
        let mut select = self.connection.prepare(
            "SELECT t.name 
            FROM tags t 
            INNER JOIN file_tags ON file_tags.tag_id = t.id 
            INNER JOIN files ON files.id = file_tags.file_id AND files.hash_sum = ?1",
        )?;
        let file_tags = select.query_map([&hash_sum], |row| row.get(0))?;
        let mut tags: Vec<String> = Vec::new();
        for tag in file_tags {
            tags.push(tag?);
        }
        Ok(tags)
    }
}

fn reference_file_tag(tx: &Transaction, file_id: i32, tag_id: i32) -> Result<()> {
    tx.execute(
        "INSERT INTO file_tags (file_id, tag_id) VALUES (?1, ?2)",
        (file_id, tag_id),
    )?;
    Ok(())
}

fn create_tag(tx: &Transaction, tag: &str) -> Result<i32> {
    let mut insert = tx.prepare(
        "INSERT INTO tags (name) 
                VALUES (?1) 
                RETURNING id",
    )?;
    Ok(insert.query_one([tag], |row| row.get(0))?)
}

fn does_tag_exist_by_name(conn: &Connection, tag: &str) -> Result<bool> {
    Ok(conn
        .prepare(
            "SELECT * FROM tags 
                WHERE name = ?1",
        )?
        .exists([&tag])?)
}

fn get_file_id(conn: &Connection, hash_sum: &str) -> Result<Option<i32>> {
    let mut select = conn.prepare(
        "SELECT id FROM files 
            WHERE hash_sum = ?1",
    )?;
    Ok(select.query_one([&hash_sum], |row| row.get(0)).optional()?)
}

fn create_file(tx: &Transaction, file_path: &str, file_name: &str, hash_sum: &str) -> Result<i32> {
    let mut insert = tx.prepare(
        "INSERT INTO files (path, name, hash_sum) 
                        VALUES (?1, ?2, ?3) 
                        RETURNING id",
    )?;
    Ok(insert.query_one((file_path, file_name, hash_sum), |row| row.get(0))?)
}
