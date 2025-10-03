use crate::db::{
    Database, File,
    tables::{
        file_tags::{get_file_tag_ids_by_id, unreference_file_tag},
        tags::get_tag_by_name,
    },
};
use anyhow::{Result, bail};
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
    pub fn get_files(&self) -> Result<Vec<String>> {
        get_all_files_path(&self.connection)
    }
    pub fn untag_file(&mut self, file: &File, tag_names: &[String]) -> Result<()> {
        let tx = self.connection.transaction()?;
        let mut db_tag_ids = vec![];

        for tag_name in tag_names {
            let Some(tag) = get_tag_by_name(&tx, tag_name)? else {
                bail!("Could not find such tag in database: {tag_name}");
            };
            debug!("found tag_id {}", tag.id);

            db_tag_ids.push(tag);
        }
        let Some(file_id) = get_file_id(&tx, &file.fingerprint_hash)? else {
            bail!("Could not find such file in database");
        };
        debug!("found file_id {file_id}");

        let file_tag_ids = get_file_tag_ids_by_id(&tx, file_id)?;
        for tag in &db_tag_ids {
            if file_tag_ids.contains(&tag.id) {
                unreference_file_tag(&tx, file_id, tag.id)?;
            } else {
                bail!("File did not have such tag: {}", tag.name);
            }
        }

        if file_tag_ids.len() == db_tag_ids.len() {
            delete_file(&tx, file_id)?;
        }

        tx.commit()?;

        Ok(())
    }
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

    Ok(query
        .query_map([], |row| row.get(0))?
        .filter_map(Result::ok)
        .collect())
}

pub fn get_all_file_ids_without_tags(conn: &Connection) -> Result<Vec<i32>> {
    let mut query = conn.prepare(
        "SELECT f.id 
            FROM files f
                LEFT JOIN file_tags ft ON f.id = ft.file_id
            WHERE ft.file_id IS NULL",
    )?;

    Ok(query
        .query_map([], |row| row.get(0))?
        .filter_map(Result::ok)
        .collect())
}
