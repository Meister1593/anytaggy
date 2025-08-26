use rusqlite::{Connection, Transaction};
use tracing::debug;
use std::fmt::Write as _;

use anyhow::Result;

pub fn reference_file_tag(tx: &Transaction, file_id: i32, tag_id: i32) -> Result<()> {
    tx.execute(
        "INSERT INTO file_tags (file_id, tag_id) VALUES (?1, ?2)",
        (file_id, tag_id),
    )?;
    debug!("referenced {file_id} with {tag_id}");
    Ok(())
}

pub fn get_file_tags(conn: &Connection, hash_sum: &str) -> Result<Vec<String>> {
    let mut select: rusqlite::Statement<'_> = conn.prepare(
        "SELECT t.name 
        FROM tags t 
        INNER JOIN file_tags ON file_tags.tag_id = t.id 
        INNER JOIN files ON files.id = file_tags.file_id AND files.hash_sum = ?1",
    )?;
    let file_tags = select.query_map([&hash_sum], |row| row.get(0))?;
    let mut tags: Vec<String> = Vec::new();
    for tag in file_tags {
        tags.push(tag?);
    }
    Ok(tags)
}

pub fn get_files_by_tags(conn: &Connection, tags: Vec<String>) -> Result<Vec<String>> {
    let mut query = String::from(
        "SELECT f.path 
        FROM files f 
        INNER JOIN file_tags ON file_tags.file_id = f.id 
        INNER JOIN tags ON tags.id = file_tags.tag_id",
    );
    for tag in tags {
        write!(query, " AND tags.name = '{tag}'")?;
    }
    let mut select = conn.prepare(&query)?;
    let file_tags = select.query_map([], |row| row.get(0))?;
    let mut paths: Vec<String> = Vec::new();
    for path in file_tags {
        paths.push(path?);
    }
    Ok(paths)
}
