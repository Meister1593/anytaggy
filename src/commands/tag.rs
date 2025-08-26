use std::path::Path;

use anyhow::Context;
use anyhow::Result;

use crate::db::Database;

pub fn tag(db: &mut Database, file_path: &Path, tags: Vec<String>) -> Result<()> {
    let file_name = file_path
        .file_name()
        .context("couldn't retrieve file name from path")?
        .display()
        .to_string();
    let hash = super::get_file_hash(file_path)?;
    let file_path = file_path.display().to_string();

    db.tag_file(&file_path, &file_name, &hash, tags)?;

    Ok(())
}
