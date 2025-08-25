use std::{fs::File, io::Write};

use assert_cmd::Command;
use rand::prelude::*;
use rusqlite::Connection;
use temp_dir::TempDir;

use crate::{commands, db::Database};

fn cargo_bin_cmd() -> Command {
    Command::cargo_bin("anytaggy").unwrap()
}

#[test]
fn blank_test() {
    let mut cmd = cargo_bin_cmd();
    cmd.assert().failure().code(2);
}

#[test]
fn tag_cmd() {
    let mut cmd = cargo_bin_cmd();

    let temp_dir = TempDir::new().unwrap();
    let temp_db = temp_dir.path().join("tmp_db.db");
    let temp_tag_file = temp_dir.path().join("temp_tag_file");
    File::create(&temp_tag_file).unwrap();

    let assert = cmd
        .arg("--database-path")
        .arg(&temp_db)
        .arg("tag")
        .arg(&temp_tag_file)
        .arg("--tags")
        .arg("test,test2,test3")
        .assert();
    assert.success();

    let mut cmd = cargo_bin_cmd();
    let assert = cmd
        .arg("--database-path")
        .arg(&temp_db)
        .arg("tags")
        .arg(&temp_tag_file)
        .assert();
    assert.success().stdout("test, test2, test3\n");
}

#[test]
fn tag_and_tags() {
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

    commands::tag::tag(&mut db, tag_file.clone(), test_tags.clone()).unwrap();
    let tags = commands::tags::tags(&db, tag_file.clone()).unwrap();
    assert_eq!(test_tags.join(", "), tags)
}

// todo: test cases with trimming to be added