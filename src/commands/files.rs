use crate::db::Database;
use anyhow::Result;

pub fn get_file_paths(db: &Database, tags: Vec<String>) -> Result<String> {
    let files = db.get_files_by_tag(tags)?;
    
    Ok(files.join("\n"))
}
