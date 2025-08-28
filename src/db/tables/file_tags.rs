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

pub fn get_file_tags_by_hash(conn: &Connection, hash: &str) -> Result<Vec<String>> {
    let mut statement = conn.prepare(
        "SELECT t.name 
        FROM tags t 
        INNER JOIN file_tags ON file_tags.tag_id = t.id 
        INNER JOIN files ON files.id = file_tags.file_id AND files.hash = ?1",
    )?;
    let file_tags = statement.query_map([&hash], |row| row.get(0))?;
    let mut tags: Vec<String> = Vec::new();
    for tag in file_tags {
        tags.push(tag?);
    }
    Ok(tags)
}

pub fn get_file_tag_ids_by_id(conn: &Connection, file_id: i32) -> Result<Vec<i32>> {
    let mut statement = conn.prepare(
        "SELECT t.id 
        FROM tags t 
        INNER JOIN file_tags ON file_tags.tag_id = t.id 
        INNER JOIN files ON files.id = file_tags.file_id AND files.id = ?1",
    )?;
    let file_tags = statement.query_map([&file_id], |row| row.get(0))?;
    let mut tag_ids: Vec<i32> = Vec::new();
    for tag_id in file_tags {
        tag_ids.push(tag_id?);
    }
    Ok(tag_ids)
}

pub fn get_file_paths_by_tags_and_op(conn: &Connection, tags: Vec<String>) -> Result<Vec<String>> {
    if tags.is_empty() {
        return Ok(vec![]);
    }
    let mut fin_tags = vec![];
    for tag in tags {
        fin_tags.push(format!("'{tag}'"));
    }
    // adapted from: https://dba.stackexchange.com/questions/267559/how-to-filter-multiple-many-to-many-relationship-based-on-multiple-tags#
    let query = format!(
        "
        select f.path
        from files f
        where f.id in (
            select ft.file_id
            from file_tags ft
                join tags t on ft.tag_id = t.id
            where t.name in ({})
            group by ft.file_id
            having count(*) = {}
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
