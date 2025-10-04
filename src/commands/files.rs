use crate::db::Database;
use anyhow::Result;

pub fn get_file_paths(db: &Database, tag_names: &[&str]) -> Result<Option<String>> {
    let files = db.get_files_by_tag(tag_names)?;
    if files.is_empty() {
        Ok(None)
    } else {
        Ok(Some(files.join("\n")))
    }
}

pub fn get_files(db: &Database) -> Result<Option<String>> {
    let files = db.get_files()?;

    if files.is_empty() {
        Ok(None)
    } else {
        Ok(Some(files.join("\n")))
    }
}
