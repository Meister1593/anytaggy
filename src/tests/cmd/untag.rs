use temp_dir::TempDir;

use crate::tests::{cmd::cargo_bin_cmd, two_files_multiple_tags_prepare};

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
        .arg("untag")
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
    assert.success().stdout("\n".to_string());
}

#[test]
fn files_clean_after_unreference() {
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
        .arg("untag")
        .arg(&tag_file)
        .arg("--tags")
        .arg("test,test2,test3")
        .assert();
    assert.success();

    let mut cmd = cargo_bin_cmd();
    let assert = cmd
        .arg("--database-path")
        .arg(&db_path)
        .arg("files")
        .assert();
    assert.success().stdout("\n".to_string());
}
