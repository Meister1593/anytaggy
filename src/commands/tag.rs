use crate::{AppError, db::Database};
use std::path::Path;

pub fn tag_file(db: &mut Database, file_path: &Path, tag_names: &[&str]) -> Result<(), AppError> {
    let file = super::create_file_struct_from_path(file_path)?;

    db.tag_file(&file, tag_names)?;

    Ok(())
}
