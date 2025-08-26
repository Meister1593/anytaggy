mod cmd;

use crate::{commands, db::Database};
use rand::prelude::*;
use rusqlite::Connection;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use temp_dir::TempDir;

pub(super) fn create_random_file(dir_path: &Path, name: &str) -> PathBuf {
    let tag_file = dir_path.join(name);
    let mut file = File::create(&tag_file).unwrap();
    let mut rng = rand::rng();
    file.write_all(&rng.random::<u128>().to_le_bytes()).unwrap();
    tag_file
}

#[test]
fn tag_file() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("tmp_db.db");
    let tag_file = temp_dir.path().join("temp_tag_file");
    let mut file = File::create(&tag_file).unwrap();
    let mut rng = rand::rng();
    file.write_all(&rng.random::<u128>().to_le_bytes()).unwrap();
    let test_tags: Vec<String> = vec!["test".into(), "test2".into(), "test3".into()];

    // Database preparation
    let connection = Connection::open(db_path).unwrap();
    let mut db = Database::new(connection);

    commands::tag::tag(&mut db, &tag_file, test_tags.clone()).unwrap();
    let tags = commands::tags::tags(&db, &tag_file).unwrap();
    assert_eq!(test_tags.join(","), tags);
}

#[test]
fn tag_and_files() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("tmp_db.db");
    let tag_file_1 = create_random_file(temp_dir.path(), "temp_tag_file_1");
    let tag_file_2 = create_random_file(temp_dir.path(), "temp_tag_file_2");
    let test_tags: Vec<String> = vec!["test".into()];
    let test_files = format!("{}\n{}", tag_file_1.display(), tag_file_2.display());

    // Database preparation
    let connection = Connection::open(db_path).unwrap();
    let mut db = Database::new(connection);

    commands::tag::tag(&mut db, &tag_file_1, test_tags.clone()).unwrap();
    commands::tag::tag(&mut db, &tag_file_2, test_tags.clone()).unwrap();
    let files = commands::files::files(&db, test_tags).unwrap();
    assert_eq!(test_files, files);
}

// todo: test cases with trimming to be added
