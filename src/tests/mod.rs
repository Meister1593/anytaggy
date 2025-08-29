mod cmd;
mod files;
mod rm_tags;
mod tag;
mod tags;
mod untag;

use rand::prelude::*;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use temp_dir::TempDir;

pub fn create_random_file(dir_path: &Path, name: &str) -> PathBuf {
    let tag_file = dir_path.join(name);
    let mut file = File::create(&tag_file).unwrap();
    let mut rng = rand::rng();
    file.write_all(&rng.random::<u128>().to_le_bytes()).unwrap();
    tag_file
}

pub fn two_files_multiple_tags_prepare(
    temp_dir: &TempDir,
) -> (PathBuf, PathBuf, PathBuf, Vec<String>, Vec<String>) {
    let db_path = temp_dir.path().join("tmp_db.db");
    let tag_file_1 = create_random_file(temp_dir.path(), "temp_tag_file_1");
    let tag_file_2 = create_random_file(temp_dir.path(), "temp_tag_file_2");
    let test_tags_1: Vec<String> = vec!["test".into(), "test2".into(), "test3".into()];
    let test_tags_2: Vec<String> = vec!["test3".into(), "test4".into(), "test5".into()];
    (db_path, tag_file_1, tag_file_2, test_tags_1, test_tags_2)
}

// todo: test cases with trimming to be added
