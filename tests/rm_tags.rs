mod common;

use crate::common::two_files_multiple_tags_prepare;
use anytaggy::{AppError, Args, Command, db::DatabaseError, entrypoint};

#[test]
fn no_rm_tags_database() {
    let (_, _, _, _, _, _temp_dir) = two_files_multiple_tags_prepare();

    let args = Args {
        database_path: None,
        command: Command::RmTags { tags: vec![] },
    };
    let out = entrypoint(args);
    assert!(matches!(out, Err(AppError::DatabaseNotFound)));
}

#[test]
fn no_rm_tags_tags() {
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
        command: Command::RmTags { tags: vec![] },
    };
    let out = entrypoint(args);
    assert!(matches!(out, Err(AppError::NoTagsSpecified)));
}

#[test]
fn rm_tags() {
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

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::RmTags {
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
    assert_eq!(None, out);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Files {
            tags: Some(test_tags),
        },
    };
    let out = entrypoint(args).unwrap();
    assert_eq!(None, out);
}

#[test]
fn no_such_tag() {
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
        command: Command::RmTags {
            tags: vec!["random-tag".into()],
        },
    };
    let out = entrypoint(args);
    assert!(matches!(
        out,
        Err(AppError::Database(DatabaseError::NoSuchTag(_))) // todo: check name for tag, matches checks for pattern - not structure
    ));
}
