use std::process::ExitCode;

use temp_dir::TempDir;

use crate::{Args, entrypoint};

#[test]
fn delete_file_tag() {
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

    let args = Args {
        database_path: db_path.clone(),
        command: crate::Command::Untag {
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
    assert_eq!(Some(String::new()), out);
    assert_eq!(ExitCode::SUCCESS, exit_code);
}

#[test]
fn files_clean_after_delete_untag() {
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

    let args = Args {
        database_path: db_path.clone(),
        command: crate::Command::Untag {
            file_path: tag_file.clone(),
            tags: test_tags.clone(),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    let args = Args {
        database_path: db_path.clone(),
        command: crate::Command::Files {
            tags: Some(test_tags.clone()),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(Some(String::new()), out);
    assert_eq!(ExitCode::SUCCESS, exit_code);
}
