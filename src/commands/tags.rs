use std::path::PathBuf;

use crate::db::Database;

pub fn tags(db: &Database, file_path: PathBuf) -> anyhow::Result<Vec<String>> {
    let hash_sum = super::get_file_hash(file_path)?;
    db.get_tags(&hash_sum)
}
