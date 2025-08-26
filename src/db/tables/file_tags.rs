use rusqlite::{Connection, Transaction};

use anyhow::Result;

pub fn reference_file_tag(tx: &Transaction, file_id: i32, tag_id: i32) -> Result<()> {
    tx.execute(
        "INSERT INTO file_tags (file_id, tag_id) VALUES (?1, ?2)",
        (file_id, tag_id),
    )?;
    Ok(())
}

pub fn get_file_tags(conn: &Connection, hash_sum: &str) -> Result<Vec<String>> {
    let mut select = conn.prepare(
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
