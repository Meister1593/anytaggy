use std::path::PathBuf;

use crate::db::Database;

pub fn tags(db: &Database, file_path: PathBuf) -> anyhow::Result<String> {
    let hash_sum = super::get_file_hash(file_path)?;
    Ok(db.get_tags(&hash_sum)?.join(", ").to_string())
}
