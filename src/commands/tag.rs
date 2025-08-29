use anyhow::Result;
use std::path::Path;

use crate::db::Database;

pub fn tag_file(db: &mut Database, file_path: &Path, tag_names: &[String]) -> Result<()> {
    let file = super::prepare_file_arg(file_path)?;

    db.tag_file(&file, tag_names)?;

    Ok(())
}
