use crate::{AppError, db::Database};

pub fn rm_tags(db: &mut Database, tag_names: &[&str]) -> Result<(), AppError> {
    db.delete_tags(tag_names)?;

    Ok(())
}
