use crate::{AppError, db::Database};
use std::path::Path;
use tracing::debug;

pub fn get_file_tags(db: &Database, file_path: &Path) -> Result<Option<String>, AppError> {
    debug!("file_path: {}", file_path.display());

    let contents_hash = super::get_file_contents_hash(file_path)?;
    debug!("contents_hash: {contents_hash}");

    let fingerprint_hash =
        super::get_fingerprint_hash(&contents_hash, &file_path.display().to_string())?;
    debug!("fingerprint_hash: {fingerprint_hash}");

    let file_tags = db.get_file_tags_by_hash(&fingerprint_hash)?;
    debug!("file_tags: {file_tags:?}");

    if file_tags.is_empty() {
        Ok(None)
    } else {
        Ok(Some(file_tags.join(",").to_string()))
    }
}

pub fn get_all_tags(db: &Database) -> Result<Option<String>, AppError> {
    let file_tags = db.get_all_tags()?;
    debug!("file_tags: {file_tags:?}");

    if file_tags.is_empty() {
        Ok(None)
    } else {
        Ok(Some(file_tags.join(",").to_string()))
    }
}
