use tracing::warn;

use crate::{AppError, db::Database};
use std::path::Path;

pub fn tag_file(db: &mut Database, file_path: &Path, tag_names: &[&str]) -> Result<(), AppError> {
    let file = super::create_file_struct_from_path(file_path)?;

    let db_files = db.get_files_by_contents_hash(&file.contents_hash)?;
    if !db_files.is_empty() {
        let affected_file_paths: Vec<String> =
            db_files.iter().map(|file| file.path.clone()).collect();
        warn!(
            "Warning: file with same hash exists, moving files ({}) will cause it to be deleted from database on repair.",
            affected_file_paths.join("; ")
        )
    }
    db.tag_file(&file, tag_names)?;

    Ok(())
}
