use anyhow::Result;
use rusqlite::{Connection, OptionalExtension, Transaction};
use tracing::debug;

#[allow(unused)]
#[derive(Debug)]
pub(in crate::db) struct DbTag {
    pub id: i32,
    pub name: String,
}

pub fn create_tag(tx: &Transaction, tag_name: &str) -> Result<DbTag> {
    let mut insert = tx.prepare(
        "INSERT INTO tags (name) 
             VALUES (?1) 
             RETURNING id, name",
    )?;

    let db_tag = insert.query_one([tag_name], |row| {
        Ok(DbTag {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;
    debug!("created tag {db_tag:?}");

    Ok(db_tag)
}

pub fn get_tags(conn: &Connection) -> Result<Vec<String>> {
    let mut query = conn.prepare("SELECT name FROM tags")?;
    let mut tag_names: Vec<String> = Vec::new();
    for tag_name in query.query_map([], |row| row.get(0))? {
        tag_names.push(tag_name?);
    }

    Ok(tag_names)
}

pub fn get_tag_id_by_name(conn: &Connection, tag_name: &str) -> Result<Option<i32>> {
    let mut query = conn.prepare(
        "SELECT id FROM tags 
             WHERE name = ?1",
    )?;

    Ok(query.query_one([tag_name], |row| row.get(0)).optional()?)
}
