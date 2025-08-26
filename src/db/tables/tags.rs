use anyhow::Result;
use rusqlite::{Connection, OptionalExtension, Transaction};
use tracing::debug;

pub fn create_tag(tx: &Transaction, tag: &str) -> Result<i32> {
    let mut insert = tx.prepare(
        "INSERT INTO tags (name) 
                VALUES (?1) 
                RETURNING id",
    )?;
    let tag_id = insert.query_one([tag], |row| row.get(0))?;
    debug!("created tag {tag} with id {tag_id}");
    Ok(tag_id)
}

pub fn get_tag_id_by_name(conn: &Connection, tag: &str) -> Result<Option<i32>> {
    let mut query = conn.prepare(
        "SELECT id FROM tags 
            WHERE name = ?1",
    )?;
    Ok(query.query_one([tag], |row| row.get(0)).optional()?)
}
