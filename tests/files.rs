mod common;

use crate::common::two_files_multiple_tags_prepare;
use anytaggy::{Args, Command, entrypoint};
use std::process::ExitCode;

#[test]
fn no_files_database() {
    let (_, _, _, _, _, _temp_dir) = two_files_multiple_tags_prepare();

    let args = Args {
        database_path: None,
        command: Command::Files { tags: None },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(Some("ERROR: Database file could not be found".into()), out);
    assert_eq!(ExitCode::FAILURE, exit_code);
}

#[test]
fn files_joined_tag() {
    // Test data
    let (db_path, tag_file_1, tag_file_2, test_tags_1, test_tags_2, _temp_dir) =
        two_files_multiple_tags_prepare();

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Tag {
            file_path: tag_file_1.clone(),
            tags: test_tags_1.clone(),
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
        command: Command::Files {
            tags: Some(vec!["test3".into()]),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(
        Some(format!(
            "{}\n{}",
            tag_file_1.display(),
            tag_file_2.display()
        )),
        out
    );
    assert_eq!(ExitCode::SUCCESS, exit_code);
}

#[test]
fn files_left_tag() {
    // Test data
    let (db_path, tag_file_1, tag_file_2, test_tags_1, test_tags_2, _temp_dir) =
        two_files_multiple_tags_prepare();

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Tag {
            file_path: tag_file_1.clone(),
            tags: test_tags_1.clone(),
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
        command: Command::Files {
            tags: Some(vec!["test".into(), "test2".into()]),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(Some(tag_file_1.display().to_string()), out);
    assert_eq!(ExitCode::SUCCESS, exit_code);
}

#[test]
fn files_right_tag() {
    // Test data
    let (db_path, tag_file_1, tag_file_2, test_tags_1, test_tags_2, _temp_dir) =
        two_files_multiple_tags_prepare();

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Tag {
            file_path: tag_file_1.clone(),
            tags: test_tags_1.clone(),
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
        command: Command::Files {
            tags: Some(vec!["test4".into()]),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(Some(tag_file_2.display().to_string()), out);
    assert_eq!(ExitCode::SUCCESS, exit_code);
}

#[test]
fn files_neither_tag() {
    // Test data
    let (db_path, tag_file_1, tag_file_2, test_tags_1, test_tags_2, _temp_dir) =
        two_files_multiple_tags_prepare();

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Tag {
            file_path: tag_file_1.clone(),
            tags: test_tags_1.clone(),
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
        command: Command::Files {
            tags: Some([&test_tags_1[..], &test_tags_2[..]].concat().clone()),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);
}

#[test]
fn get_all_files() {
    // Test data
    let (db_path, tag_file_1, tag_file_2, test_tags_1, test_tags_2, _temp_dir) =
        two_files_multiple_tags_prepare();

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Tag {
            file_path: tag_file_1.clone(),
            tags: test_tags_1.clone(),
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
        command: Command::Files { tags: None },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(
        Some(format!(
            "{}\n{}",
            tag_file_1.display(),
            tag_file_2.display()
        )),
        out
    );
    assert_eq!(ExitCode::SUCCESS, exit_code);
}

#[test]
fn no_tags_specified() {
    // Test data
    let (db_path, tag_file_1, _, test_tags_1, _, _temp_dir) = two_files_multiple_tags_prepare();

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Tag {
            file_path: tag_file_1.clone(),
            tags: test_tags_1.clone(),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Files { tags: Some(vec![]) },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(Some("ERROR: No tags specified".into()), out);
    assert_eq!(ExitCode::FAILURE, exit_code);
}
