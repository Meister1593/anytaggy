use anyhow::Result;
use rusqlite::{Connection, OptionalExtension, Transaction};
use tracing::debug;

use crate::db::File;

pub fn create_file(tx: &Transaction, file: &File) -> Result<i32> {
    let mut insert = tx.prepare(
        "INSERT INTO files (path, name, contents_hash, hash) 
                        VALUES (?1, ?2, ?3, ?4) 
                        RETURNING id",
    )?;
    
    let file_id = insert.query_one(
        (file.path, file.name, file.contents_hash, file.hash),
        |row| row.get(0),
    )?;
    debug!("created file {} with id {file_id}", file.name);

    Ok(file_id)
}

pub fn get_file_id(conn: &Connection, hash: &str) -> Result<Option<i32>> {
    let mut select = conn.prepare(
        "SELECT id FROM files 
            WHERE hash = ?1",
    )?;

    Ok(select.query_one([&hash], |row| row.get(0)).optional()?)
}
