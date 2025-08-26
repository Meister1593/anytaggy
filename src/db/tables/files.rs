use anyhow::Result;
use rusqlite::{Connection, OptionalExtension, Transaction};
use tracing::debug;

pub fn create_file(
    tx: &Transaction,
    file_path: &str,
    file_name: &str,
    hash: &str,
) -> Result<i32> {
    let mut insert = tx.prepare(
        "INSERT INTO files (path, name, hash) 
                        VALUES (?1, ?2, ?3) 
                        RETURNING id",
    )?;
    let file_id = insert.query_one((file_path, file_name, hash), |row| row.get(0))?;
    debug!("created file {file_name} with id {file_id}");
    Ok(file_id)
}

pub fn get_file_id(conn: &Connection, hash: &str) -> Result<Option<i32>> {
    let mut select = conn.prepare(
        "SELECT id FROM files 
            WHERE hash = ?1",
    )?;
    Ok(select.query_one([&hash], |row| row.get(0)).optional()?)
}
