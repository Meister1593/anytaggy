use predicates::prelude::predicate;
use temp_dir::TempDir;

use crate::tests::{cmd::cargo_bin_cmd, two_files_multiple_tags_prepare};

#[test]
fn files_joined_tag() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file_1, tag_file_2, test_tags_1, test_tags_2) =
        two_files_multiple_tags_prepare(&temp_dir);

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
}

#[test]
fn files_left_tag() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file_1, tag_file_2, test_tags_1, test_tags_2) =
        two_files_multiple_tags_prepare(&temp_dir);

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
        .arg("test")
        .assert();
    assert
        .success()
        .stdout(predicate::eq(format!("{}\n", tag_file_1.display())));
}

#[test]
fn files_right_tag() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file_1, tag_file_2, test_tags_1, test_tags_2) =
        two_files_multiple_tags_prepare(&temp_dir);

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
        .arg("test4")
        .assert();
    assert
        .success()
        .stdout(predicate::eq(format!("{}\n", tag_file_2.display())));
}

#[test]
fn files_neither_tag() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file_1, tag_file_2, test_tags_1, test_tags_2) =
        two_files_multiple_tags_prepare(&temp_dir);

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
        .arg([&test_tags_1[..], &test_tags_2[..]].concat().join(" "))
        .assert();
    assert.success().stdout(predicate::eq("\n"));
}

#[test]
fn delete_file_tag() {
    let mut cmd = cargo_bin_cmd();

    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file, _, test_tags, _) = two_files_multiple_tags_prepare(&temp_dir);

    let assert = cmd
        .arg("--database-path")
        .arg(&db_path)
        .arg("tag")
        .arg(&tag_file)
        .arg("--tags")
        .arg("test,test2,test3")
        .assert();
    assert.success();

    let mut cmd = cargo_bin_cmd();
    let assert = cmd
        .arg("--database-path")
        .arg(&db_path)
        .arg("tags")
        .arg(&tag_file)
        .assert();
    assert
        .success()
        .stdout(format!("{}\n", test_tags.join(",")));

    let mut cmd = cargo_bin_cmd();
    let assert = cmd
        .arg("--database-path")
        .arg(&db_path)
        .arg("tag")
        .arg(&tag_file)
        .arg("--tags")
        .arg("test,test2,test3")
        .arg("--delete")
        .assert();
    assert.success();

    let mut cmd = cargo_bin_cmd();
    let assert = cmd
        .arg("--database-path")
        .arg(&db_path)
        .arg("tags")
        .arg(&tag_file)
        .assert();
    assert.success().stdout("\n".to_string());
}
