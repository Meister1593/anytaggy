mod common;

use crate::common::two_files_multiple_tags_prepare;
use anytaggy::{Args, Command, entrypoint};
use serial_test::serial;

#[test]
#[serial]
fn repair_database_on_renamed_file() {
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

    let new_tag_file = tag_file
        .parent()
        .unwrap()
        .to_path_buf()
        .join("new_tag_file");
    std::fs::rename(tag_file.clone(), new_tag_file.clone()).unwrap();

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Repair {},
    };
    let out = entrypoint(args).unwrap();
    assert_eq!(None, out);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Files {
            tags: Some(test_tags.clone()),
        },
    };
    let out = entrypoint(args).unwrap();
    assert_eq!(Some(format!("{}", new_tag_file.display())), out);
}

#[test]
#[serial]
fn repair_database_on_moved_file() {
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

    let new_dir = _temp_dir.path().join("random_folder");
    std::fs::create_dir(new_dir.clone()).unwrap();
    let tag_file_new_path = new_dir.join(tag_file.file_name().unwrap());
    std::fs::rename(tag_file.clone(), tag_file_new_path.clone()).unwrap();

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Repair {},
    };
    let out = entrypoint(args).unwrap();
    assert_eq!(None, out);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Files {
            tags: Some(test_tags.clone()),
        },
    };
    let out = entrypoint(args).unwrap();
    assert_eq!(Some(format!("{}", tag_file_new_path.display())), out);
}

#[test]
#[serial]
fn repair_database_on_moved_and_renamed_file() {
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

    let new_dir = _temp_dir.path().join("random_folder");
    std::fs::create_dir(new_dir.clone()).unwrap();
    let new_tag_file_new_path = new_dir.join("new_tag_file");
    std::fs::rename(tag_file.clone(), new_tag_file_new_path.clone()).unwrap();

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Repair {},
    };
    let out = entrypoint(args).unwrap();
    assert_eq!(None, out);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Files {
            tags: Some(test_tags.clone()),
        },
    };
    let out = entrypoint(args).unwrap();
    assert_eq!(Some(format!("{}", new_tag_file_new_path.display())), out);
}

#[test]
#[serial]
fn repair_database_on_deleted_file() {
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

    std::fs::remove_file(tag_file.clone()).unwrap();

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Repair {},
    };
    let out = entrypoint(args).unwrap();
    assert_eq!(None, out);

    let args = Args {
        database_path: Some(db_path.clone()),
        command: Command::Files {
            tags: Some(test_tags.clone()),
        },
    };
    let out = entrypoint(args).unwrap();
    assert_eq!(None, out);
}
