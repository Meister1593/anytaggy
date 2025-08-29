use crate::db::Database;
use anyhow::Result;

pub fn get_file_paths(db: &Database, tag_names: &[String]) -> Result<String> {
    let files = db.get_files_by_tag(tag_names)?;

    Ok(files.join("\n"))
}

pub fn get_files(db: &Database) -> Result<String> {
    let files = db.get_files()?;

    Ok(files.join("\n"))
}
