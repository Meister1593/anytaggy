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

pub(super) fn two_files_multiple_tags_prepare(
    temp_dir: &TempDir,
) -> (PathBuf, PathBuf, PathBuf, Vec<String>, Vec<String>) {
    let db_path = temp_dir.path().join("tmp_db.db");
    let tag_file_1 = create_random_file(temp_dir.path(), "temp_tag_file_1");
    let tag_file_2 = create_random_file(temp_dir.path(), "temp_tag_file_2");
    let test_tags_1: Vec<String> = vec!["test".into(), "test2".into(), "test3".into()];
    let test_tags_2: Vec<String> = vec!["test3".into(), "test4".into(), "test5".into()];
    (db_path, tag_file_1, tag_file_2, test_tags_1, test_tags_2)
}

#[test]
fn tag_file() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file, _, test_tags, _) = two_files_multiple_tags_prepare(&temp_dir);

    // Database preparation
    let connection = Connection::open(db_path).unwrap();
    let mut db = Database::new(connection);

    commands::tag::tag_file(&mut db, &tag_file, test_tags.clone()).unwrap();
    let tags = commands::tags::get_file_tags(&db, &tag_file).unwrap();
    assert_eq!(test_tags.join(","), tags);
}

#[test]
fn files_joined_tag() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file_1, tag_file_2, test_tags_1, test_tags_2) =
        two_files_multiple_tags_prepare(&temp_dir);

    // Database preparation
    let connection = Connection::open(db_path).unwrap();
    let mut db = Database::new(connection);

    commands::tag::tag_file(&mut db, &tag_file_1, test_tags_1.clone()).unwrap();
    commands::tag::tag_file(&mut db, &tag_file_2, test_tags_2.clone()).unwrap();
    assert_eq!(
        format!("{}\n{}", tag_file_1.display(), tag_file_2.display()),
        commands::files::get_file_paths(&db, vec!["test3".into()]).unwrap()
    );
}

#[test]
fn files_left_tag() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file_1, tag_file_2, test_tags_1, test_tags_2) =
        two_files_multiple_tags_prepare(&temp_dir);

    // Database preparation
    let connection = Connection::open(db_path).unwrap();
    let mut db = Database::new(connection);

    commands::tag::tag_file(&mut db, &tag_file_1, test_tags_1.clone()).unwrap();
    commands::tag::tag_file(&mut db, &tag_file_2, test_tags_2.clone()).unwrap();
    assert_eq!(
        tag_file_1.display().to_string(),
        commands::files::get_file_paths(&db, vec!["test".into(), "test2".into()]).unwrap()
    );
}

#[test]
fn files_right_tag() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file_1, tag_file_2, test_tags_1, test_tags_2) =
        two_files_multiple_tags_prepare(&temp_dir);

    // Database preparation
    let connection = Connection::open(db_path).unwrap();
    let mut db = Database::new(connection);

    commands::tag::tag_file(&mut db, &tag_file_1, test_tags_1.clone()).unwrap();
    commands::tag::tag_file(&mut db, &tag_file_2, test_tags_2.clone()).unwrap();
    assert_eq!(
        tag_file_2.display().to_string(),
        commands::files::get_file_paths(&db, vec!["test4".into()]).unwrap()
    );
}

#[test]
fn files_neither_tag() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file_1, tag_file_2, test_tags_1, test_tags_2) =
        two_files_multiple_tags_prepare(&temp_dir);

    // Database preparation
    let connection = Connection::open(db_path).unwrap();
    let mut db = Database::new(connection);

    commands::tag::tag_file(&mut db, &tag_file_1, test_tags_1.clone()).unwrap();
    commands::tag::tag_file(&mut db, &tag_file_2, test_tags_2.clone()).unwrap();
    assert_eq!(
        "",
        commands::files::get_file_paths(&db, [&test_tags_1[..], &test_tags_2[..]].concat())
            .unwrap()
    );
}

#[test]
fn get_tags() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file_1, _, test_tags_1, _) = two_files_multiple_tags_prepare(&temp_dir);

    // Database preparation
    let connection = Connection::open(db_path).unwrap();
    let mut db = Database::new(connection);

    commands::tag::tag_file(&mut db, &tag_file_1, test_tags_1.clone()).unwrap();
    assert_eq!(
        test_tags_1.join(","),
        commands::tags::get_all_tags(&db).unwrap()
    );
}

// todo: test cases with trimming to be added
