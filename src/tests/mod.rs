use std::fs::File;

use assert_cmd::Command;
use rusqlite::Connection;
use temp_dir::TempDir;

use crate::{commands, db::Database};

fn cargo_bin_cmd() -> Command {
    Command::cargo_bin("anytaggy").unwrap()
}

#[test]
fn blank_test() {
    let mut cmd = cargo_bin_cmd();
    cmd.assert().failure();
}

#[test]
fn tag_cmd() {
    let mut cmd = cargo_bin_cmd();

    let temp_dir = TempDir::new().unwrap();
    let temp_db = temp_dir.path().join("tmp_db.sql");
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

    let assert = cmd
        .arg("--database-path")
        .arg(&temp_db)
        .arg("tags")
        .arg(&temp_tag_file)
        .assert();
    assert.success().stdout("test,test2,test3");
}

#[test]
fn tag() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("tmp_db.db");
    let tag_file = temp_dir.path().join("temp_tag_file");
    File::create(&tag_file).unwrap();
    let connection = Connection::open(db_path).unwrap();
    let mut db = Database::new(connection);
    db.apply_migrations();

    commands::tag::tag(db, tag_file, vec!["test".to_string()]).unwrap();
}
