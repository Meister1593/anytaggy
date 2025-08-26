use anyhow::Result;

use crate::db::Database;

pub fn files(db: &Database, tags: Vec<String>) -> Result<String> {
    let files = db.get_files_by_tag(tags)?;
    Ok(files.join("\n"))
}
