mod common;

use crate::common::{create_random_file, temp_dir_prepare, two_files_multiple_tags_prepare};
use anytaggy::{AppError, Args, Command, DATABASE_FILENAME, entrypoint};
use serial_test::serial;
use std::{fs::create_dir, path::PathBuf};

#[test]
#[serial]
fn create_and_find_database_in_parent() {
    // Test data
    let temp_dir = temp_dir_prepare();
    let db_path = temp_dir.path().join(DATABASE_FILENAME);
    let subfolder = temp_dir.path().join("folder");
    create_dir(&subfolder).unwrap();
    let tag_file = create_random_file(&subfolder, "temp_tag_file");
    let test_tags: Vec<String> = vec!["test".into()];

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Tag {
            file_path: tag_file,
            tags: test_tags,
        },
    };
    let out = entrypoint(args).unwrap();
    assert_eq!(None, out);

    std::env::set_current_dir(subfolder).unwrap();
    let args = Args {
        database_path: None,
        command: Command::Tags { file_path: None },
    };
    let out = entrypoint(args).unwrap();
    assert_eq!(Some("test".into()), out);
}

#[test]
#[serial]
fn create_and_find_database_in_current_dir() {
    let (_, tag_file_1, _, test_tags_1, _, _temp_dir) = two_files_multiple_tags_prepare();

    let args = Args {
        database_path: None,
        command: Command::Tag {
            file_path: tag_file_1,
            tags: test_tags_1.clone(),
        },
    };
    let out = entrypoint(args).unwrap();
    assert_eq!(None, out);

    let args = Args {
        database_path: None,
        command: Command::Tags { file_path: None },
    };
    let out = entrypoint(args).unwrap();
    assert_eq!(Some(test_tags_1.join(",")), out);
}

#[test]
#[serial]
fn dont_find_database() {
    let (_, _, _, _, _, _temp_dir) = two_files_multiple_tags_prepare();

    let args = Args {
        database_path: Some(PathBuf::default()),
        command: Command::Tags { file_path: None },
    };
    let out = entrypoint(args);
    assert!(matches!(out, Err(AppError::DatabaseNotFound)));
}
