use anyhow::Result;
use rusqlite::{Connection, Transaction};
use std::vec;
use tracing::debug;

pub fn reference_file_tag(tx: &Transaction, file_id: i32, tag_id: i32) -> Result<()> {
    tx.execute(
        "INSERT INTO file_tags (file_id, tag_id) VALUES (?1, ?2)",
        (file_id, tag_id),
    )?;
    debug!("referenced {file_id} with {tag_id}");
    Ok(())
}

pub fn get_file_tags_by_hash(conn: &Connection, fingerprint_hash: &str) -> Result<Vec<String>> {
    let mut statement = conn.prepare(
        "SELECT t.name 
        FROM tags t 
            INNER JOIN file_tags ON file_tags.tag_id = t.id 
            INNER JOIN files ON file_tags.file_id = files.id
        WHERE files.fingerprint_hash = ?1",
    )?;
    let file_tag_names = statement.query_map([&fingerprint_hash], |row| row.get(0))?;
    let mut tags: Vec<String> = Vec::new();
    for tag_name in file_tag_names {
        tags.push(tag_name?);
    }
    Ok(tags)
}

pub fn get_file_tag_ids_by_id(conn: &Connection, file_id: i32) -> Result<Vec<i32>> {
    let mut statement = conn.prepare(
        "SELECT t.id 
        FROM tags t 
            INNER JOIN file_tags ON file_tags.tag_id = t.id 
            INNER JOIN files ON file_tags.file_id = files.id
        WHERE files.id = ?1",
    )?;
    let file_tags = statement.query_map([&file_id], |row| row.get(0))?;
    let mut tag_ids: Vec<i32> = Vec::new();
    for tag_id in file_tags {
        tag_ids.push(tag_id?);
    }
    Ok(tag_ids)
}

pub fn get_file_paths_by_tags_and_op(
    conn: &Connection,
    tag_names: Vec<String>,
) -> Result<Vec<String>> {
    if tag_names.is_empty() {
        return Ok(vec![]);
    }
    let mut fin_tags = vec![];
    for tag_name in tag_names {
        fin_tags.push(format!("'{tag_name}'"));
    }
    // adapted from: https://dba.stackexchange.com/questions/267559/how-to-filter-multiple-many-to-many-relationship-based-on-multiple-tags#
    let query = format!(
        "
        SELECT f.path
        FROM files f
        WHERE f.id IN (
            SELECT ft.file_id
            FROM file_tags ft
                INNER JOIN tags t on ft.tag_id = t.id
            WHERE t.name IN ({})
            GROUP BY ft.file_id
            HAVING COUNT(*) = {}
            )",
        fin_tags.join(","),
        fin_tags.len()
    );
    let mut statement = conn.prepare(&query)?;
    let file_tags = statement.query_map([], |row| row.get(0))?;
    let mut paths: Vec<String> = Vec::new();
    for path in file_tags {
        paths.push(path?);
    }
    Ok(paths)
}
