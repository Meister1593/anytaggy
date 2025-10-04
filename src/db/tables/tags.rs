use crate::db::{
    Database, DatabaseError, File,
    tables::{
        file_tags::{get_file_tag_ids_by_id, reference_file_tag},
        files::{create_file, get_file_id},
    },
};
use rusqlite::{Connection, OptionalExtension, Transaction};
use tracing::{debug, info};

#[allow(unused)]
#[derive(Debug)]
pub(in crate::db) struct DbTag {
    pub id: i32,
    pub name: String,
}

impl Database {
    pub fn tag_file(&mut self, file: &File, tag_names: &[&str]) -> Result<(), DatabaseError> {
        let tx = self.connection.transaction()?;

        let file_id = get_file_id(&tx, &file.fingerprint_hash)?
            .map_or_else(|| create_file(&tx, file).map(|f| f.id), Ok)?;
        debug!("file_id: {file_id}");

        let file_tag_ids = get_file_tag_ids_by_id(&tx, file_id)?;
        for tag_name in tag_names {
            let tag_name = tag_name.trim();
            let tag_id = get_tag_id_by_name(&tx, tag_name)?.map_or_else(
                || {
                    let tag_id = create_tag(&tx, tag_name).map(|tag| tag.id);
                    info!("created tag: {tag_name}");
                    tag_id
                },
                Ok,
            )?;
            debug!("tag_id: {tag_id}");

            if !file_tag_ids.contains(&tag_id) {
                reference_file_tag(&tx, file_id, tag_id)?;
            }
        }

        tx.commit()?;

        Ok(())
    }

    pub fn get_all_tags(&self) -> Result<Vec<String>, DatabaseError> {
        get_tag_names(&self.connection)
    }

    pub fn delete_tags(&mut self, names: &[&str]) -> Result<(), DatabaseError> {
        let tx = self.connection.transaction()?;
        for name in names {
            let Some(tag) = get_tag_by_name(&tx, name)? else {
                return Err(DatabaseError::NoSuchTag(name.to_string()));
            };
            delete_tag(&tx, tag.id)?;
        }

        tx.commit()?;
        Ok(())
    }
}

pub fn get_tag_by_name(conn: &Connection, name: &str) -> Result<Option<DbTag>, DatabaseError> {
    let mut query = conn.prepare(
        "SELECT * FROM tags 
             WHERE name = ?1",
    )?;

    Ok(query
        .query_one([name], |row| {
            Ok(DbTag {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
        .optional()?)
}

fn delete_tag(tx: &Transaction, id: i32) -> Result<(), DatabaseError> {
    tx.execute(
        "DELETE FROM tags
             WHERE id = ?1",
        (id,),
    )?;
    debug!("deleted tag with id: {id}");

    Ok(())
}

fn create_tag(tx: &Transaction, name: &str) -> Result<DbTag, DatabaseError> {
    let mut insert = tx.prepare(
        "INSERT INTO tags (name) 
             VALUES (?1) 
             RETURNING id, name",
    )?;

    let db_tag = insert.query_one([name], |row| {
        Ok(DbTag {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;
    debug!("created tag {db_tag:?}");

    Ok(db_tag)
}

fn get_tag_names(conn: &Connection) -> Result<Vec<String>, DatabaseError> {
    let mut query = conn.prepare("SELECT name FROM tags")?;

    Ok(query
        .query_map([], |row| row.get(0))?
        .filter_map(Result::ok)
        .collect())
}

fn get_tag_id_by_name(conn: &Connection, name: &str) -> Result<Option<i32>, DatabaseError> {
    let mut query = conn.prepare(
        "SELECT id FROM tags 
             WHERE name = ?1",
    )?;

    Ok(query.query_one([name], |row| row.get(0)).optional()?)
}
