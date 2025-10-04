mod common;

use crate::common::{create_random_file, two_files_multiple_tags_prepare};
use anytaggy::{AppError, Args, Command, entrypoint};
use temp_dir::TempDir;

#[test]
fn no_tag_tags() {
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
        command: Command::Tag {
            file_path: tag_file.clone(),
            tags: vec![],
        },
    };
    let out = entrypoint(args);
    assert!(matches!(out, Err(AppError::NoTagsSpecified)));
}

#[test]
fn tag_file() {
    // Test data
    let (db_path, tag_file, _, test_tags, test_tags_1, _temp_dir) =
        two_files_multiple_tags_prepare();

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
        command: Command::Tag {
            file_path: tag_file.clone(),
            tags: test_tags_1.clone(),
        },
    };
    let out = entrypoint(args).unwrap();
    assert_eq!(None, out);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Tags {
            file_path: Some(tag_file),
        },
    };
    let out = entrypoint(args).unwrap();
    // this needed just to not have duplicates
    let mut out_tags = [test_tags, test_tags_1].concat();
    out_tags.dedup();
    assert_eq!(Some(out_tags.join(",")), out);
}

#[test]
fn tag_file_in_parent_directory_without_db() {
    let temp_dir = TempDir::new().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();
    let db_parent_path = temp_dir.path().join("parent");
    std::fs::create_dir(&db_parent_path).unwrap();
    let db_path = &db_parent_path.join("tmp_db.db");
    let tag_file_1 = create_random_file(temp_dir.path(), "temp_tag_file_1");

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Tag {
            file_path: tag_file_1.clone(),
            tags: vec!["test".into()],
        },
    };
    let out = entrypoint(args);
    assert!(matches!(out, Err(AppError::FileOutsideStructure)));
}
