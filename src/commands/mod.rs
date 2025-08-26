use std::{
    fs::File,
    io::{self},
    path::PathBuf,
};

use sha2::Digest;

pub mod tag;
pub mod tags;

pub fn get_file_hash(file_path: PathBuf) -> Result<String, anyhow::Error> {
    let mut hasher = sha2::Sha256::new();
    let mut file = File::open(&file_path)?;
    io::copy(&mut file, &mut hasher)?;
    let result = hasher.finalize();
    Ok(format!("{result:x}"))
}
