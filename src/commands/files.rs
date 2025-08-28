use crate::db::Database;
use anyhow::Result;

pub fn get_file_paths(db: &Database, tag_names: Vec<String>) -> Result<String> {
    let files = db.get_files_by_tag(tag_names)?;

    Ok(files.join("\n"))
}
