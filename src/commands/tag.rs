use anyhow::Context;
use anyhow::Result;
use std::path::Path;
use tracing::debug;

use crate::db::Database;
use crate::db::File;

pub fn tag_file(db: &mut Database, file_path: &Path, tags: Vec<String>) -> Result<()> {
    let name = file_path
        .file_name()
        .context("couldn't retrieve file name from path")?
        .to_str()
        .context("couldn't convert file name to str")?;
    debug!("name: {name}");

    let path = file_path
        .to_str()
        .context("couldn't convert file path to str")?;
    debug!("path: {path}");

    let contents_hash = super::get_file_contents_hash(file_path)?;
    debug!("contents_hash: {contents_hash}");

    let hash = super::get_file_hash(&contents_hash, path)?;
    debug!("hash: {hash}");

    db.tag_file(
        &File {
            path,
            name,
            contents_hash: &contents_hash,
            hash: &hash,
        },
        tags,
    )?;

    Ok(())
}
