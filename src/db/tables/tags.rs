use anyhow::Result;
use rusqlite::{Connection, Transaction};

pub fn create_tag(tx: &Transaction, tag: &str) -> Result<i32> {
    let mut insert = tx.prepare(
        "INSERT INTO tags (name) 
                VALUES (?1) 
                RETURNING id",
    )?;
    Ok(insert.query_one([tag], |row| row.get(0))?)
}

pub fn does_tag_exist_by_name(conn: &Connection, tag: &str) -> Result<bool> {
    Ok(conn
        .prepare(
            "SELECT * FROM tags 
                WHERE name = ?1",
        )?
        .exists([&tag])?)
}
