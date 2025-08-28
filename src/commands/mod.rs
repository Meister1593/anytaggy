pub mod files;
pub mod tag;
pub mod tags;

use anyhow::Result;
use sha2::Digest;
use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

pub fn get_file_contents_hash(file_path: &Path) -> Result<String> {
    let mut hasher = sha2::Sha256::new();
    let mut file = File::open(file_path)?;
    io::copy(&mut file, &mut hasher)?;
    let result = hasher.finalize();
    
    Ok(format!("{result:x}"))
}

pub fn get_file_hash(file_contents_hash: &str, file_path_string: &str) -> Result<String> {
    let mut hasher = sha2::Sha256::new();
    let new_hash = format!("{file_contents_hash}_{file_path_string}");
    hasher.write_all(new_hash.as_bytes())?;
    let result = hasher.finalize();
    
    Ok(format!("{result:x}"))
}
