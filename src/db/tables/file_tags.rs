use crate::db::{Database, DatabaseError};
use rusqlite::{Connection, Transaction};
use tracing::debug;

impl Database {
    pub fn get_file_tags_by_hash(
        &self,
        fingerprint_hash: &str,
    ) -> Result<Vec<String>, DatabaseError> {
        get_file_tags_by_hash(&self.connection, fingerprint_hash)
    }

    pub fn get_file_paths_by_tags_names(
        &self,
        tag_names: &[&str],
    ) -> Result<Vec<String>, DatabaseError> {
        get_file_paths_by_tags_names(&self.connection, tag_names)
    }
}

pub fn unreference_file_tag(
    tx: &Transaction,
    file_id: i32,
    tag_id: i32,
) -> Result<(), DatabaseError> {
    tx.execute(
        "DELETE FROM file_tags
             WHERE file_id = ?1 AND tag_id = ?2",
        (file_id, tag_id),
    )?;
    debug!("unreferenced {file_id} with {tag_id}");

    Ok(())
}

pub fn reference_file_tag(
    tx: &Transaction,
    file_id: i32,
    tag_id: i32,
) -> Result<(), DatabaseError> {
    tx.execute(
        "INSERT INTO file_tags (file_id, tag_id) 
             VALUES (?1, ?2)",
        (file_id, tag_id),
    )?;
    debug!("referenced {file_id} with {tag_id}");

    Ok(())
}

fn get_file_tags_by_hash(
    conn: &Connection,
    fingerprint_hash: &str,
) -> Result<Vec<String>, DatabaseError> {
    let mut statement = conn.prepare(
        "SELECT t.name 
        FROM tags t 
            INNER JOIN file_tags ON file_tags.tag_id = t.id 
            INNER JOIN files ON file_tags.file_id = files.id
        WHERE files.fingerprint_hash = ?1",
    )?;

    Ok(statement
        .query_map([&fingerprint_hash], |row| row.get(0))?
        .filter_map(Result::ok)
        .collect())
}

pub fn get_tag_ids_by_file_id(conn: &Connection, file_id: i32) -> Result<Vec<i32>, DatabaseError> {
    let mut statement = conn.prepare(
        "SELECT t.id 
        FROM tags t 
            INNER JOIN file_tags ON file_tags.tag_id = t.id 
            INNER JOIN files ON file_tags.file_id = files.id
        WHERE files.id = ?1",
    )?;
    Ok(statement
        .query_map([&file_id], |row| row.get(0))?
        .filter_map(Result::ok)
        .collect())
}

fn get_file_paths_by_tags_names(
    conn: &Connection,
    tag_names: &[&str],
) -> Result<Vec<String>, DatabaseError> {
    let tag_names: Vec<String> = tag_names
        .iter()
        .map(|tag_name| format!("'{tag_name}'"))
        .collect();
    let query = format!(
        "SELECT f.path
         FROM files f
             INNER JOIN file_tags ft on ft.file_id = f.id
             INNER JOIN tags t on ft.tag_id = t.id
         WHERE t.name IN ({})
         GROUP BY f.id
         HAVING COUNT(*) = {}",
        tag_names.join(","),
        tag_names.len()
    );
    let mut statement = conn.prepare(&query)?;
    Ok(statement
        .query_map([], |row| row.get(0))?
        .filter_map(Result::ok)
        .collect())
}
