use crate::db::{
    Database, DatabaseError, File,
    tables::{
        file_tags::{get_file_tag_ids_by_id, unreference_file_tag},
        tags::get_tag_by_name,
    },
};
use rusqlite::{Connection, OptionalExtension, Transaction};
use tracing::debug;

#[allow(unused)]
#[derive(Debug)]
pub(in crate::db) struct DbFile {
    pub id: i32,
    pub path: String,
    pub name: String,
    pub contents_hash: String,
    pub fingerprint_hash: String,
}

impl Database {
    pub fn get_files(&self) -> Result<Vec<String>, DatabaseError> {
        get_all_files_path(&self.connection).map_err(DatabaseError::DatabaseInternal)
    }
    pub fn untag_file(&mut self, file: &File, tag_names: &[&str]) -> Result<(), DatabaseError> {
        let tx = self.connection.transaction()?;

        let Some(file_id) = get_file_id(&tx, &file.fingerprint_hash)? else {
            return Err(DatabaseError::NoSuchFile);
        };
        debug!("found file_id {file_id}");

        let mut unreferenced_tags_count = 0;
        let file_tag_ids = get_file_tag_ids_by_id(&tx, file_id)?;
        for tag_name in tag_names {
            let Some(tag) = get_tag_by_name(&tx, tag_name)? else {
                return Err(DatabaseError::NoSuchTag((*tag_name).into()));
            };
            debug!("found tag_id {}", tag.id);

            if file_tag_ids.contains(&tag.id) {
                unreference_file_tag(&tx, file_id, tag.id)?;
                unreferenced_tags_count += 1;
            } else {
                return Err(DatabaseError::NoSuchTagOnFile(tag.name));
            }
        }

        // if we deleted all tags from file
        if file_tag_ids.len() == unreferenced_tags_count {
            // delete the file from database as unnecessary
            delete_file(&tx, file_id)?;
        }

        tx.commit()?;

        Ok(())
    }
}

pub fn delete_file(tx: &Transaction, id: i32) -> Result<(), rusqlite::Error> {
    tx.execute(
        "DELETE FROM files
             WHERE id = ?1",
        (id,),
    )?;
    debug!("deleted file with id: {id}");

    Ok(())
}

pub fn create_file(tx: &Transaction, file: &File) -> Result<DbFile, rusqlite::Error> {
    let mut insert = tx.prepare(
        "INSERT INTO files (path, name, contents_hash, fingerprint_hash) 
             VALUES (?1, ?2, ?3, ?4) 
             RETURNING id, path, name, contents_hash, fingerprint_hash",
    )?;

    let db_file = insert.query_one(
        (
            &file.path,
            &file.name,
            &file.contents_hash,
            &file.fingerprint_hash,
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

pub fn get_file_id(
    conn: &Connection,
    fingerprint_hash: &str,
) -> Result<Option<i32>, rusqlite::Error> {
    let mut select = conn.prepare(
        "SELECT id 
            FROM files 
            WHERE fingerprint_hash = ?1",
    )?;

    select
        .query_one([&fingerprint_hash], |row| row.get(0))
        .optional()
}

fn get_all_files_path(conn: &Connection) -> Result<Vec<String>, rusqlite::Error> {
    let mut query = conn.prepare(
        "SELECT path 
            FROM files",
    )?;

    Ok(query
        .query_map([], |row| row.get(0))?
        .filter_map(Result::ok)
        .collect())
}
