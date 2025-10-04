mod common;

use anytaggy::{AppError, Args, Command, entrypoint};
use temp_dir::TempDir;

use crate::common::{create_random_file, two_files_multiple_tags_prepare};

#[test]
fn no_tags_database() {
    let (_, _, _, _, _, _temp_dir) = two_files_multiple_tags_prepare();

    let args = Args {
        database_path: None,
        command: Command::Tags { file_path: None },
    };
    let out = entrypoint(args);
    assert!(matches!(out, Err(AppError::DatabaseNotFound)));
}

#[test]
fn get_tags() {
    // Test data
    let (db_path, tag_file, _, test_tags, _, _temp_dir) = two_files_multiple_tags_prepare();

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Tag {
            file_path: tag_file.clone(),
            tags: test_tags.clone(),
        },
    };
    let out = entrypoint(args).unwrap();
    assert_eq!(None, out);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Tags {
            file_path: Some(tag_file.clone()),
        },
    };
    let out = entrypoint(args).unwrap();
    assert_eq!(Some(test_tags.join(",")), out);
}

#[test]
fn get_all_tags() {
    // Test data
    let (db_path, tag_file, _, test_tags, _, _temp_dir) = two_files_multiple_tags_prepare();

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Tag {
            file_path: tag_file.clone(),
            tags: test_tags.clone(),
        },
    };
    let out = entrypoint(args).unwrap();
    assert_eq!(None, out);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Tags { file_path: None },
    };
    let out = entrypoint(args).unwrap();
    assert_eq!(Some(test_tags.join(",")), out);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::RmTags { tags: test_tags },
    };
    let out = entrypoint(args).unwrap();
    assert_eq!(None, out);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Tags { file_path: None },
    };
    let out = entrypoint(args).unwrap();
    assert_eq!(None, out);
}

#[test]
fn tags_file_in_parent_directory_without_db() {
    let temp_dir = TempDir::new().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();
    let db_parent_path = temp_dir.path().join("parent");
    std::fs::create_dir(&db_parent_path).unwrap();
    let db_path = &db_parent_path.join("tmp_db.db");
    let tag_file_1 = create_random_file(temp_dir.path(), "temp_tag_file_1");
    let tag_file_ok = create_random_file(&db_parent_path, "temp_tag_file_ok");

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Tag {
            file_path: tag_file_ok.clone(),
            tags: vec!["test".into()],
        },
    };
    let out = entrypoint(args).unwrap();
    assert_eq!(None, out);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Tags {
            file_path: Some(tag_file_1.clone()),
        },
    };
    let out = entrypoint(args);
    assert!(matches!(out, Err(AppError::FileOutsideStructure)));
}
