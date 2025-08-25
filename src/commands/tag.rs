use std::path::PathBuf;

use anyhow::Context;

use crate::db::Database;

pub fn tag(db: &mut Database, file_path: PathBuf, tags: Vec<String>) -> anyhow::Result<()> {
    let file_name = file_path
        .file_name()
        .context("couldn't retrieve file name from path")?
        .display()
        .to_string();
    let hash_sum = super::get_file_hash(file_path.clone())?;
    let file_path = file_path.display().to_string();

    db.tag(&file_path, &file_name, &hash_sum, tags)?;

    Ok(())
}
