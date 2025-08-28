use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::File;
use temp_dir::TempDir;

fn cargo_bin_cmd() -> Command {
    Command::cargo_bin("anytaggy").unwrap()
}
#[test]
fn tag_and_files_cmd() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let temp_dir = temp_dir.dont_delete_on_drop();
    let db_path = temp_dir.path().join("tmp_db.db");
    let tag_file_1 = super::create_random_file(temp_dir.path(), "temp_tag_file_1");
    let tag_file_2 = super::create_random_file(temp_dir.path(), "temp_tag_file_2");
    let test_tags_1: Vec<String> = vec!["test".into(), "test2".into(), "test3".into()];
    let test_tags_2: Vec<String> = vec!["test3".into(), "test4".into(), "test5".into()];

    let mut cmd = cargo_bin_cmd();
    let assert = cmd
        .arg("--database-path")
        .arg(&db_path)
        .arg("tag")
        .arg(&tag_file_1)
        .arg("--tags")
        .arg(test_tags_1.join(","))
        .assert();
    assert.success();

    let mut cmd = cargo_bin_cmd();
    let assert = cmd
        .arg("--database-path")
        .arg(&db_path)
        .arg("tag")
        .arg(&tag_file_2)
        .arg("--tags")
        .arg(test_tags_2.join(","))
        .assert();
    assert.success();

    let mut cmd = cargo_bin_cmd();
    let assert = cmd
        .arg("--database-path")
        .arg(&db_path)
        .arg("files")
        .arg("test3")
        .assert();
    assert.success().stdout(predicate::eq(format!(
        "{}\n{}\n",
        tag_file_1.display(),
        tag_file_2.display()
    )));

    let mut cmd = cargo_bin_cmd();
    let assert = cmd
        .arg("--database-path")
        .arg(&db_path)
        .arg("files")
        .arg("test")
        .assert();
    assert
        .success()
        .stdout(predicate::eq(format!("{}\n", tag_file_1.display())));

    let mut cmd = cargo_bin_cmd();
    let assert = cmd
        .arg("--database-path")
        .arg(&db_path)
        .arg("files")
        .arg("test4")
        .assert();
    assert
        .success()
        .stdout(predicate::eq(format!("{}\n", tag_file_2.display())));

    let mut cmd = cargo_bin_cmd();
    let assert = cmd
        .arg("--database-path")
        .arg(&db_path)
        .arg("files")
        .arg([&test_tags_1[..], &test_tags_2[..]].concat().join(" "))
        .assert();
    assert.success().stdout(predicate::eq("\n"));
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
