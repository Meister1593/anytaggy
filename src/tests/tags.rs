use std::{path::PathBuf, process::ExitCode};

use crate::{Args, entrypoint};
use temp_dir::TempDir;

#[test]
fn no_tags_database() {
    let args = Args {
        database_path: PathBuf::new(),
        command: crate::Command::Tags { file_path: None },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(Some("ERROR: Database file could not be found".into()), out);
    assert_eq!(ExitCode::FAILURE, exit_code);
}

#[test]
fn get_tags() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file, _, test_tags, _) = super::two_files_multiple_tags_prepare(&temp_dir);

    let args = Args {
        database_path: db_path.clone(),
        command: crate::Command::Tag {
            file_path: tag_file.clone(),
            tags: test_tags.clone(),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    let args = Args {
        database_path: db_path.clone(),
        command: crate::Command::Tags {
            file_path: Some(tag_file.clone()),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(Some(test_tags.join(",")), out);
    assert_eq!(ExitCode::SUCCESS, exit_code);
}

#[test]
fn get_all_tags() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file, _, test_tags, _) = super::two_files_multiple_tags_prepare(&temp_dir);

    let args = Args {
        database_path: db_path.clone(),
        command: crate::Command::Tag {
            file_path: tag_file.clone(),
            tags: test_tags.clone(),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    let args = Args {
        database_path: db_path.clone(),
        command: crate::Command::Tags { file_path: None },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(Some(test_tags.join(",")), out);
    assert_eq!(ExitCode::SUCCESS, exit_code);
}
