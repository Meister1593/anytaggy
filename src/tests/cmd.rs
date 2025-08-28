use assert_cmd::Command;
use predicates::prelude::*;
use temp_dir::TempDir;

fn cargo_bin_cmd() -> Command {
    Command::cargo_bin("anytaggy").unwrap()
}
#[test]
fn tag_and_files_cmd() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file_1, tag_file_2, test_tags_1, test_tags_2) =
        super::two_files_multiple_tags_prepare(&temp_dir);

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
    let (db_path, tag_file, _, test_tags, _) = super::two_files_multiple_tags_prepare(&temp_dir);

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
}
