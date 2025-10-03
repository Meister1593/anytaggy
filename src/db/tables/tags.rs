use crate::db::{
    Database, File,
    tables::{
        file_tags::{get_file_tag_ids_by_id, reference_file_tag},
        files::{create_file, delete_file, get_all_file_ids_without_tags, get_file_id},
    },
};
use anyhow::{Result, bail};
use rusqlite::{Connection, OptionalExtension, Transaction};
use tracing::{debug, info};

#[allow(unused)]
#[derive(Debug)]
pub(in crate::db) struct DbTag {
    pub id: i32,
    pub name: String,
}

impl Database {
    pub fn tag_file(&mut self, file: &File, tag_names: &[String]) -> Result<()> {
        let tx = self.connection.transaction()?;
        let mut db_tag_ids = vec![];

        for tag_name in tag_names {
            let tag_name = tag_name.trim();
            let tag_id = if let Some(tag_id) = get_tag_id_by_name(&tx, tag_name)? {
                tag_id
            } else {
                let db_tag = create_tag(&tx, tag_name)?;
                info!("created tag: {tag_name}");
                db_tag.id
            };
            debug!("tag_id: {tag_id}");
            db_tag_ids.push(tag_id);
        }

        // todo: this looks kinda ugly, might be better to use unwrap_or_else (but then no automatic ?)
        let file_id = if let Some(file_id) = get_file_id(&tx, &file.fingerprint_hash)? {
            file_id
        } else {
            create_file(&tx, file)?.id
        };
        debug!("file_id: {file_id}");

        let file_tag_ids: Vec<i32> = get_file_tag_ids_by_id(&tx, file_id)?;
        for tag_id in db_tag_ids {
            if !file_tag_ids.contains(&tag_id) {
                reference_file_tag(&tx, file_id, tag_id)?;
            }
        }

        tx.commit()?;

        Ok(())
    }

    pub fn get_all_tags(&self) -> Result<Vec<String>> {
        get_tag_names(&self.connection)
    }

    pub fn delete_tags(&mut self, names: &[String]) -> Result<()> {
        let tx = self.connection.transaction()?;
        for name in names {
            let Some(tag) = get_tag_by_name(&tx, name)? else {
                bail!("Could not find such tag in database: {name}");
            };
            delete_tag(&tx, tag.id)?;
        }

        for id in get_all_file_ids_without_tags(&tx)? {
            delete_file(&tx, id)?;
        }

        tx.commit()?;
        Ok(())
    }
}

pub fn delete_tag(tx: &Transaction, id: i32) -> Result<()> {
    tx.execute(
        "DELETE FROM tags
             WHERE id = ?1",
        (id,),
    )?;
    debug!("deleted tag with id: {id}");

    Ok(())
}

pub fn create_tag(tx: &Transaction, name: &str) -> Result<DbTag> {
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

pub fn get_tag_names(conn: &Connection) -> Result<Vec<String>> {
    let mut query = conn.prepare("SELECT name FROM tags")?;

    Ok(query
        .query_map([], |row| row.get(0))?
        .filter_map(Result::ok)
        .collect())
}

pub fn get_tag_id_by_name(conn: &Connection, name: &str) -> Result<Option<i32>> {
    let mut query = conn.prepare(
        "SELECT id FROM tags 
             WHERE name = ?1",
    )?;

    Ok(query.query_one([name], |row| row.get(0)).optional()?)
}

pub fn get_tag_by_name(conn: &Connection, name: &str) -> Result<Option<DbTag>> {
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
