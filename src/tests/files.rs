use std::process::ExitCode;

use crate::{Args, entrypoint};
use temp_dir::TempDir;

#[test]
fn files_joined_tag() {
    // Test data
    let temp_dir: TempDir = TempDir::new().unwrap();
    let (db_path, tag_file_1, tag_file_2, test_tags_1, test_tags_2) =
        super::two_files_multiple_tags_prepare(&temp_dir);

    let args = Args {
        database_path: db_path.clone(),
        command: crate::Command::Tag {
            file_path: tag_file_1.clone(),
            tags: test_tags_1.clone(),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);
    let args = Args {
        database_path: db_path.clone(),
        command: crate::Command::Tag {
            file_path: tag_file_2.clone(),
            tags: test_tags_2.clone(),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    let args = Args {
        database_path: db_path.clone(),
        command: crate::Command::Files {
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
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file_1, tag_file_2, test_tags_1, test_tags_2) =
        super::two_files_multiple_tags_prepare(&temp_dir);

    let args = Args {
        database_path: db_path.clone(),
        command: crate::Command::Tag {
            file_path: tag_file_1.clone(),
            tags: test_tags_1.clone(),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);
    let args = Args {
        database_path: db_path.clone(),
        command: crate::Command::Tag {
            file_path: tag_file_2.clone(),
            tags: test_tags_2.clone(),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    let args = Args {
        database_path: db_path.clone(),
        command: crate::Command::Files {
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
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file_1, tag_file_2, test_tags_1, test_tags_2) =
        super::two_files_multiple_tags_prepare(&temp_dir);

    let args = Args {
        database_path: db_path.clone(),
        command: crate::Command::Tag {
            file_path: tag_file_1.clone(),
            tags: test_tags_1.clone(),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);
    let args = Args {
        database_path: db_path.clone(),
        command: crate::Command::Tag {
            file_path: tag_file_2.clone(),
            tags: test_tags_2.clone(),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    let args = Args {
        database_path: db_path.clone(),
        command: crate::Command::Files {
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
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file_1, tag_file_2, test_tags_1, test_tags_2) =
        super::two_files_multiple_tags_prepare(&temp_dir);

    let args = Args {
        database_path: db_path.clone(),
        command: crate::Command::Tag {
            file_path: tag_file_1.clone(),
            tags: test_tags_1.clone(),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);
    let args = Args {
        database_path: db_path.clone(),
        command: crate::Command::Tag {
            file_path: tag_file_2.clone(),
            tags: test_tags_2.clone(),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    let args = Args {
        database_path: db_path.clone(),
        command: crate::Command::Files {
            tags: Some([&test_tags_1[..], &test_tags_2[..]].concat().clone()),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(Some(String::new()), out);
    assert_eq!(ExitCode::SUCCESS, exit_code);
}

#[test]
fn get_all_files() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file_1, tag_file_2, test_tags_1, test_tags_2) =
        super::two_files_multiple_tags_prepare(&temp_dir);

    let args = Args {
        database_path: db_path.clone(),
        command: crate::Command::Tag {
            file_path: tag_file_1.clone(),
            tags: test_tags_1.clone(),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);
    let args = Args {
        database_path: db_path.clone(),
        command: crate::Command::Tag {
            file_path: tag_file_2.clone(),
            tags: test_tags_2.clone(),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    let args = Args {
        database_path: db_path.clone(),
        command: crate::Command::Files { tags: None },
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
