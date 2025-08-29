use crate::db::Database;
use anyhow::Result;

pub fn rm_tags(db: &mut Database, tag_names: &[String]) -> Result<()> {
    db.delete_tags(tag_names)?;

    Ok(())
}
