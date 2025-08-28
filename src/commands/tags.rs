use crate::db::Database;
use anyhow::Result;
use std::path::Path;
use tracing::debug;

pub fn get_file_tags(db: &Database, file_path: &Path) -> Result<String> {
    debug!("file_path: {}", file_path.display());
    let contents_hash = super::get_file_contents_hash(file_path)?;
    let hash = super::get_file_hash(&contents_hash, &file_path.display().to_string())?;
    debug!("hash: {hash}");

    Ok(db.get_file_tags(&hash)?.join(",").to_string())
}

pub fn get_all_tags(db: &Database) -> Result<String> {
    Ok(db.get_all_tags()?.join(",").to_string())
}
