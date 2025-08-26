use std::{
    fs::File,
    io::{self},
    path::Path,
};

use anyhow::Result;
use sha2::Digest;

pub mod tag;
pub mod tags;

pub fn get_file_hash(file_path: &Path) -> Result<String> {
    let mut hasher = sha2::Sha256::new();
    let mut file = File::open(file_path)?;
    io::copy(&mut file, &mut hasher)?;
    let result = hasher.finalize();
    Ok(format!("{result:x}"))
}
