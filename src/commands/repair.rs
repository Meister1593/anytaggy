use std::path::Path;

use tracing::{debug, info, warn};
use walkdir::WalkDir;

use crate::{
    AppError,
    commands::create_file_struct_from_path,
    db::{Database, DbFile},
};

pub fn repair(db: &mut Database, search_path: &Path) -> Result<(), AppError> {
    let mut files_to_update: Vec<DbFile> = Vec::new();
    for entry in WalkDir::new(search_path) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let file = create_file_struct_from_path(&entry.into_path())?;
            let file_size = match file.size.parse::<u64>() {
                Ok(file_size) => file_size,
                Err(e) => return Err(AppError::ParseError(e)),
            };
            if file_size == 0 {
                // we cannot compare empty files with different names, it's impossible
                warn!("file ({}) has zero size, cannot be compared.", file.path);
                continue;
            }

            let db_files = db.get_files_by_contents_hash(&file.contents_hash)?;
            if db_files.len() >= 2 {
                // we cannot compare multiple conflicting hashes
                warn!("hash collision: {:?}", db_files);
                continue;
            }
            if let Some(db_file) = db_files.first() {
                debug!("found match for db file by contents hash");

                if file.fingerprint_hash.ne(&db_file.fingerprint_hash) {
                    debug!(
                        "found mismatched file: {} (stored in db) and {} (actual)",
                        &db_file.path, &file.path
                    );
                    files_to_update.push(
                        DbFile::builder()
                            .id(db_file.id)
                            .path(file.path.clone())
                            .name(file.name)
                            .contents_hash(file.contents_hash)
                            .fingerprint_hash(file.fingerprint_hash)
                            .size(file.size)
                            .build(),
                    );
                }
            }
        }
    }

    let mut files_to_delete: Vec<DbFile> = Vec::new();
    let all_db_files = db.get_all_files()?;
    for db_file in all_db_files {
        let any_updated_file_matched = files_to_update.iter().any(|file| file.id.eq(&db_file.id));
        if !Path::new(&db_file.path).exists() && !any_updated_file_matched {
            debug!("found missing file: {} (stored in db)", &db_file.path);

            files_to_delete.push(db_file);
        }
    }

    db.bulk_update_and_delete(&files_to_update, &files_to_delete)?;

    for file in files_to_update {
        let message = format!("Updated file location in DB: {}", &file.path);
        println!("{}", message);
        info!(message)
    }
    for file in files_to_delete {
        let message = format!("Cleaned file from DB: {}", &file.path);
        println!("{}", message);
        info!(message)
    }

    Ok(())
}
