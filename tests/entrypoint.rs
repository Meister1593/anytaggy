mod common;

use crate::common::create_random_file;
use anytaggy::{Args, Command, DATABASE_FILENAME, entrypoint};
use std::{fs::create_dir, path::PathBuf, process::ExitCode};
use temp_dir::TempDir;

#[test]
fn create_and_find_database_in_parent() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();

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
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    std::env::set_current_dir(subfolder).unwrap();
    let args = Args {
        database_path: None,
        command: Command::Tags { file_path: None },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(Some("test".into()), out);
    assert_eq!(ExitCode::SUCCESS, exit_code);
}

#[test]
fn create_and_find_database_in_current_dir() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();

    let tag_file = create_random_file(temp_dir.path(), "temp_tag_file");
    let test_tags: Vec<String> = vec!["test".into()];

    let args = Args {
        database_path: None,
        command: Command::Tag {
            file_path: tag_file,
            tags: test_tags,
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    let args = Args {
        database_path: None,
        command: Command::Tags { file_path: None },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(Some("test".into()), out);
    assert_eq!(ExitCode::SUCCESS, exit_code);
}

#[test]
fn dont_find_database() {
    let temp_dir = TempDir::new().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();

    let args = Args {
        database_path: Some(PathBuf::default()),
        command: Command::Tags { file_path: None },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(
        Some("ERROR: Specified database file could not be found".into()),
        out
    );
    assert_eq!(ExitCode::FAILURE, exit_code);
}
