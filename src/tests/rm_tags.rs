use std::process::ExitCode;

use temp_dir::TempDir;

use crate::{Args, entrypoint};

#[test]
fn no_rm_tags_database() {
    let temp_dir = TempDir::new().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();

    let args = Args {
        database_path: None,
        command: crate::Command::RmTags { tags: vec![] },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(Some("ERROR: Database file could not be found".into()), out);
    assert_eq!(ExitCode::FAILURE, exit_code);
}

#[test]
fn no_rm_tags_tags() {
    let temp_dir = TempDir::new().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();

    let (db_path, tag_file, _, test_tags, _, _temp_dir) = super::two_files_multiple_tags_prepare();

    let args = Args {
        database_path: Some(db_path.clone()),
        command: crate::Command::Tag {
            file_path: tag_file.clone(),
            tags: test_tags.clone(),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: crate::Command::RmTags { tags: vec![] },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(Some("ERROR: No tags specified".into()), out);
    assert_eq!(ExitCode::FAILURE, exit_code);
}

#[test]
fn rm_tags() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();

    let (db_path, tag_file, _, test_tags, _, _temp_dir) = super::two_files_multiple_tags_prepare();

    let args = Args {
        database_path: Some(db_path.clone()),
        command: crate::Command::Tag {
            file_path: tag_file.clone(),
            tags: test_tags.clone(),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: crate::Command::Tags {
            file_path: Some(tag_file.clone()),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(Some(test_tags.join(",")), out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: crate::Command::RmTags {
            tags: test_tags.clone(),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: crate::Command::Tags {
            file_path: Some(tag_file.clone()),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(Some(String::new()), out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: crate::Command::Files {
            tags: Some(test_tags),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(Some(String::new()), out);
    assert_eq!(ExitCode::SUCCESS, exit_code);
}
