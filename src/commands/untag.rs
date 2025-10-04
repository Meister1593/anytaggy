use crate::db::Database;
use anyhow::Result;
use std::path::Path;

pub fn untag_file(db: &mut Database, file_path: &Path, tag_names: &[&str]) -> Result<()> {
    let file = super::prepare_file_arg(file_path)?;
    db.untag_file(&file, tag_names)?;

    Ok(())
}
