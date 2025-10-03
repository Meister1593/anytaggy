mod common;

use crate::common::{create_random_file, two_files_multiple_tags_prepare};
use anytaggy::{Args, Command, entrypoint};
use std::{path::PathBuf, process::ExitCode};
use temp_dir::TempDir;

#[test]
fn no_tags_specified() {
    let (_, _, _, _, _, _temp_dir) = two_files_multiple_tags_prepare();

    let args = Args {
        database_path: None,
        command: Command::Untag {
            file_path: PathBuf::new(),
            tags: vec![],
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(Some("ERROR: Database file could not be found".into()), out);
    assert_eq!(ExitCode::FAILURE, exit_code);
}

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
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Untag {
            file_path: tag_file.clone(),
            tags: vec![],
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(Some("ERROR: No tags specified".into()), out);
    assert_eq!(ExitCode::FAILURE, exit_code);
}

#[test]
fn untag_file() {
    // Test data
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

    let random_tag_name = "random_tag".to_string();
    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Untag {
            file_path: tag_file.clone(),
            tags: vec![random_tag_name.clone()],
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(
        Some(format!(
            "Could not find such tag in database: {random_tag_name}"
        )),
        out
    );
    assert_eq!(ExitCode::FAILURE, exit_code);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Untag {
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
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);
}

#[test]
fn files_clean_after_delete_untag() {
    // Test data
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
        command: Command::Untag {
            file_path: tag_file.clone(),
            tags: test_tags.clone(),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Files {
            tags: Some(test_tags.clone()),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);
}

#[test]
fn no_such_file() {
    // Test data
    let (db_path, tag_file, tag_file_2, test_tags, _, _temp_dir) =
        two_files_multiple_tags_prepare();

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
        command: Command::Untag {
            file_path: tag_file_2.clone(),
            tags: test_tags.clone(),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(Some("Could not find such file in database".into()), out);
    assert_eq!(ExitCode::FAILURE, exit_code);
}

#[test]
fn no_such_tag_on_file() {
    // Test data
    let (db_path, tag_file, tag_file_2, test_tags, test_tags_2, _temp_dir) =
        two_files_multiple_tags_prepare();

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
        command: Command::Tag {
            file_path: tag_file_2.clone(),
            tags: test_tags_2.clone(),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Untag {
            file_path: tag_file.clone(),
            tags: test_tags_2.clone(),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert!(out.unwrap().starts_with("File did not have such tag: "));
    assert_eq!(ExitCode::FAILURE, exit_code);
}

#[test]
fn untag_file_in_parent_directory_without_db() {
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
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Untag {
            file_path: tag_file_1.clone(),
            tags: vec!["test".into()],
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(
        Some("ERROR: Could not access file outside of database structure".into()),
        out
    );
    assert_eq!(ExitCode::FAILURE, exit_code);
}
