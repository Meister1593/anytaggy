use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use assert_cmd::Command;
use predicates::prelude::*;
use rand::prelude::*;
use rusqlite::Connection;
use temp_dir::TempDir;

use crate::{commands, db::Database};

fn cargo_bin_cmd() -> Command {
    Command::cargo_bin("anytaggy").unwrap()
}

fn create_random_file(dir_path: &Path, name: &str) -> PathBuf {
    let tag_file = dir_path.join(name);
    let mut file = File::create(&tag_file).unwrap();
    let mut rng = rand::rng();
    file.write_all(&rng.random::<u128>().to_le_bytes()).unwrap();
    tag_file
}

#[test]
fn blank_test() {
    let mut cmd = cargo_bin_cmd();
    cmd.assert().failure().code(2);
}

#[test]
fn tag_file_cmd() {
    let mut cmd = cargo_bin_cmd();

    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("tmp_db.db");
    let temp_tag_file = temp_dir.path().join("temp_tag_file");
    File::create(&temp_tag_file).unwrap();

    let assert = cmd
        .arg("--database-path")
        .arg(&db_path)
        .arg("tag")
        .arg(&temp_tag_file)
        .arg("--tags")
        .arg("test,test2,test3")
        .assert();
    assert.success();

    let mut cmd = cargo_bin_cmd();
    let assert = cmd
        .arg("--database-path")
        .arg(&db_path)
        .arg("tags")
        .arg(&temp_tag_file)
        .assert();
    assert.success().stdout("test,test2,test3\n");
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
fn tag_and_files_cmd() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let temp_dir = temp_dir.dont_delete_on_drop();
    let db_path = temp_dir.path().join("tmp_db.db");
    let tag_file_1 = create_random_file(temp_dir.path(), "temp_tag_file_1");
    let tag_file_2 = create_random_file(temp_dir.path(), "temp_tag_file_2");
    let test_tags = "test";
    let test_files = format!("{}\n{}\n", tag_file_1.display(), tag_file_2.display());

    let mut cmd = cargo_bin_cmd();
    let assert = cmd
        .arg("--database-path")
        .arg(&db_path)
        .arg("tag")
        .arg(&tag_file_1)
        .arg("--tags")
        .arg(test_tags)
        .assert();
    assert.success();

    let mut cmd = cargo_bin_cmd();
    let assert = cmd
        .arg("--database-path")
        .arg(&db_path)
        .arg("tag")
        .arg(&tag_file_2)
        .arg("--tags")
        .arg(test_tags)
        .assert();
    assert.success();

    let mut cmd = cargo_bin_cmd();
    let assert = cmd
        .arg("--database-path")
        .arg(&db_path)
        .arg("files")
        .arg(test_tags)
        .assert();
    assert.success().stdout(predicate::eq(test_files));
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
