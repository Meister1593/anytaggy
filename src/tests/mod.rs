use std::fs::File;

use assert_cmd::Command;
use temp_dir::TempDir;

fn cargo_bin_cmd() -> Command {
    Command::cargo_bin("anytaggy").unwrap()
}

#[test]
fn blank_test() {
    let mut cmd = cargo_bin_cmd();
    cmd.assert().failure();
}

#[test]
fn tag() {
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
