use std::process::ExitCode;

use crate::{Args, entrypoint};
use temp_dir::TempDir;

#[test]
fn no_tag_tags() {
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
        database_path: db_path,
        command: crate::Command::Tag {
            file_path: tag_file.clone(),
            tags: vec![],
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(Some("ERROR: No tags specified".into()), out);
    assert_eq!(ExitCode::FAILURE, exit_code);
}

#[test]
fn tag_file() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file, _, test_tags, test_tags_1) =
        super::two_files_multiple_tags_prepare(&temp_dir);

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
        command: crate::Command::Tag {
            file_path: tag_file.clone(),
            tags: test_tags_1.clone(),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    assert_eq!(None, out);
    assert_eq!(ExitCode::SUCCESS, exit_code);

    let args = Args {
        database_path: db_path,
        command: crate::Command::Tags {
            file_path: Some(tag_file),
        },
    };
    let (out, exit_code) = entrypoint(args).unwrap();
    // this needed just to not have duplicates
    let mut out_tags = [test_tags, test_tags_1].concat();
    out_tags.dedup();
    assert_eq!(Some(out_tags.join(",")), out);
    assert_eq!(ExitCode::SUCCESS, exit_code);
}
