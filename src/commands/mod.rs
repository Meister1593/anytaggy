pub mod files;
pub mod rm_tags;
pub mod tag;
pub mod tags;
pub mod untag;

use sha2::Digest;
use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};
use tracing::debug;

use crate::AppError;

pub(super) fn get_file_contents_hash(file_path: &Path) -> Result<String, AppError> {
    let mut hasher = sha2::Sha256::new();
    let mut file = File::open(file_path)?;
    io::copy(&mut file, &mut hasher)?;
    let result = hasher.finalize();

    Ok(format!("{result:x}"))
}

pub(super) fn get_fingerprint_hash(
    file_contents_hash: &str,
    file_path_string: &str,
) -> Result<String, AppError> {
    let mut hasher = sha2::Sha256::new();
    let new_hash = format!("{file_contents_hash}_{file_path_string}");
    hasher.write_all(new_hash.as_bytes())?;
    let result = hasher.finalize();

    Ok(format!("{result:x}"))
}

pub(super) fn prepare_file_arg(file_path: &Path) -> Result<crate::db::File, AppError> {
    let name = file_path
        .file_name()
        .ok_or(AppError::CantGetFileNameFromPath)?
        .display()
        .to_string();
    debug!("name: {name}");

    let path = file_path.display().to_string();
    debug!("path: {path}");

    let contents_hash = get_file_contents_hash(file_path)?;
    debug!("contents_hash: {contents_hash}");

    let fingerprint_hash = get_fingerprint_hash(&contents_hash, &path)?;
    debug!("fingerprint_hash: {fingerprint_hash}");

    Ok(crate::db::File {
        path,
        name,
        contents_hash,
        fingerprint_hash,
    })
}
