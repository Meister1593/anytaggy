use std::path::Path;

use tracing::{debug, info};
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

            let db_file = db.get_file_by_contents_hash(&file.contents_hash)?;
            if let Some(db_file) = db_file {
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
                            .build(),
                    );
                }
            }
        }
    }

    let mut files_to_delete: Vec<DbFile> = Vec::new();
    let all_db_files = db.get_all_files()?;
    for db_file in all_db_files {
        if !Path::new(&db_file.path).exists()
            && !files_to_update
                .iter()
                .any(|file| file.contents_hash.eq(&db_file.contents_hash))
        {
            debug!("found missing file: {} (stored in db)", &db_file.path);
            files_to_delete.push(db_file);
        }
    }

    db.bulk_update_and_delete(&files_to_update, &files_to_delete)?;

    for file in files_to_update {
        info!("Updated file location in DB: {}", &file.path);
    }
    for file in files_to_delete {
        info!("Deleted file from db: {}", &file.path);
    }

    Ok(())
}
