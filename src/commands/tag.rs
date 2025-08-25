use std::{fs::File, io::Read, path::PathBuf};

use anyhow::Context;
use sha_rs::Sha;

use crate::db::Database;

pub fn tag(mut db: Database, file_path: PathBuf, tags: Vec<String>) -> anyhow::Result<()> {
    let file_name = file_path
        .file_name()
        .context("no file name")?
        .display()
        .to_string();
    let hash_sum = {
        let hasher = sha_rs::Sha256::new();
        let mut file = File::open(&file_path)?;
        let mut buf: Vec<u8> = vec![];
        file.read_to_end(&mut buf)?;
        hasher.digest(&buf)
    };
    let file_path = file_path
        .to_str()
        .context("couldn't convert file path to str")?;
    db.tag(file_path, &file_name, &hash_sum, tags)?;

    Ok(())
}
