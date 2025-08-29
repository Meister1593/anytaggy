use anyhow::Result;
use rusqlite::{Connection, OptionalExtension, Transaction};
use tracing::debug;

use crate::db::File;

#[allow(unused)]
#[derive(Debug)]
pub(in crate::db) struct DbFile {
    pub id: i32,
    pub path: String,
    pub name: String,
    pub contents_hash: String,
    pub fingerprint_hash: String,
}

pub fn delete_file(tx: &Transaction, id: i32) -> Result<()> {
    tx.execute(
        "DELETE FROM files
             WHERE id = ?1",
        (id,),
    )?;
    debug!("deleted file with id: {id}");

    Ok(())
}

pub fn create_file(tx: &Transaction, file: &File) -> Result<DbFile> {
    let mut insert = tx.prepare(
        "INSERT INTO files (path, name, contents_hash, fingerprint_hash) 
             VALUES (?1, ?2, ?3, ?4) 
             RETURNING id, path, name, contents_hash, fingerprint_hash",
    )?;

    let db_file = insert.query_one(
        (
            file.path,
            file.name,
            file.contents_hash,
            file.fingerprint_hash,
        ),
        |row| {
            Ok(DbFile {
                id: row.get(0)?,
                path: row.get(1)?,
                name: row.get(2)?,
                contents_hash: row.get(3)?,
                fingerprint_hash: row.get(4)?,
            })
        },
    )?;
    debug!("created file {file:?}");

    Ok(db_file)
}

pub fn get_file_id(conn: &Connection, fingerprint_hash: &str) -> Result<Option<i32>> {
    let mut select = conn.prepare(
        "SELECT id 
            FROM files 
            WHERE fingerprint_hash = ?1",
    )?;

    Ok(select
        .query_one([&fingerprint_hash], |row| row.get(0))
        .optional()?)
}

pub fn get_all_files_path(conn: &Connection) -> Result<Vec<String>> {
    let mut query = conn.prepare(
        "SELECT path 
            FROM files",
    )?;
    let mut paths: Vec<String> = Vec::new();
    for path in query.query_map([], |row| row.get(0))? {
        paths.push(path?);
    }
    Ok(paths)
}
