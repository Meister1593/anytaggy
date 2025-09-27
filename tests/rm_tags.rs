mod common;

use crate::common::two_files_multiple_tags_prepare;
use anytaggy::{Args, Command, entrypoint};
use std::process::ExitCode;

#[test]
fn no_rm_tags_database() {
    let (_, _, _, _, _, _temp_dir) = two_files_multiple_tags_prepare();

    let args = Args {
        database_path: None,
        command: Command::RmTags { tags: vec![] },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(Some("ERROR: Database file could not be found".into()), out);
    assert_eq!(ExitCode::FAILURE, exit_code);
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
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::RmTags { tags: vec![] },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(Some("ERROR: No tags specified".into()), out);
    assert_eq!(ExitCode::FAILURE, exit_code);
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
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Tags {
            file_path: Some(tag_file.clone()),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(Some(test_tags.join(",")), out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::RmTags {
            tags: test_tags.clone(),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Tags {
            file_path: Some(tag_file.clone()),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Files {
            tags: Some(test_tags),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);
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
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::RmTags {
            tags: vec!["random-tag".into()],
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();

    assert_eq!(
        out.unwrap(),
        "Could not find such tag in database: random-tag"
    );
    assert_eq!(ExitCode::FAILURE, exit_code);
}
