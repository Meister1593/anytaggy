use crate::{AppError, db::Database};

pub fn get_file_paths(db: &Database, tag_names: &[&str]) -> Result<Option<String>, AppError> {
    let files = db.get_file_paths_by_tags_names(tag_names)?;
    if files.is_empty() {
        Ok(None)
    } else {
        Ok(Some(files.join("\n")))
    }
}

pub fn get_files(db: &Database) -> Result<Option<String>, AppError> {
    let files = db.get_all_files_paths()?;

    if files.is_empty() {
        Ok(None)
    } else {
        Ok(Some(files.join("\n")))
    }
}
