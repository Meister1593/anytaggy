use std::{fs::File, io::Read, path::PathBuf};

use sha_rs::Sha;

pub mod tag;
pub mod tags;

pub fn get_file_hash(file_path: PathBuf) -> Result<String, anyhow::Error> {
    let hasher = sha_rs::Sha256::new();
    let mut file = File::open(&file_path)?;
    let mut buf: Vec<u8> = vec![];
    file.read_to_end(&mut buf)?;
    Ok(hasher.digest(&buf))
}
