use std::path::Path;

use crate::db::Database;
use anyhow::Result;

pub fn tags(db: &Database, file_path: &Path) -> Result<String> {
    let hash = super::get_file_hash(file_path)?;
    Ok(db.get_file_tags(&hash)?.join(",").to_string())
}
