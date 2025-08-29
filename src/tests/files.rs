use crate::{
    commands,
    db::{Database, DatabaseMode},
};
use temp_dir::TempDir;

#[test]
fn files_joined_tag() {
    // Test data
    let temp_dir: TempDir = TempDir::new().unwrap();
    let (db_path, tag_file_1, tag_file_2, test_tags_1, test_tags_2) =
        super::two_files_multiple_tags_prepare(&temp_dir);

    let mut db = Database::new(&DatabaseMode::ReadWrite, &db_path);
    commands::tag::tag_file(&mut db, &tag_file_1, &test_tags_1, false).unwrap();
    commands::tag::tag_file(&mut db, &tag_file_2, &test_tags_2, false).unwrap();

    let db = Database::new(&DatabaseMode::Read, &db_path);
    assert_eq!(
        format!("{}\n{}", tag_file_1.display(), tag_file_2.display()),
        commands::files::get_file_paths(&db, &["test3".into()]).unwrap()
    );
}

#[test]
fn files_left_tag() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file_1, tag_file_2, test_tags_1, test_tags_2) =
        super::two_files_multiple_tags_prepare(&temp_dir);

    let mut db = Database::new(&DatabaseMode::ReadWrite, &db_path);
    commands::tag::tag_file(&mut db, &tag_file_1, &test_tags_1, false).unwrap();
    commands::tag::tag_file(&mut db, &tag_file_2, &test_tags_2, false).unwrap();

    let db = Database::new(&DatabaseMode::Read, &db_path);
    assert_eq!(
        tag_file_1.display().to_string(),
        commands::files::get_file_paths(&db, &["test".into(), "test2".into()]).unwrap()
    );
}

#[test]
fn files_right_tag() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file_1, tag_file_2, test_tags_1, test_tags_2) =
        super::two_files_multiple_tags_prepare(&temp_dir);

    let mut db = Database::new(&DatabaseMode::ReadWrite, &db_path);
    commands::tag::tag_file(&mut db, &tag_file_1, &test_tags_1, false).unwrap();
    commands::tag::tag_file(&mut db, &tag_file_2, &test_tags_2, false).unwrap();

    let db = Database::new(&DatabaseMode::Read, &db_path);
    assert_eq!(
        tag_file_2.display().to_string(),
        commands::files::get_file_paths(&db, &["test4".into()]).unwrap()
    );
}

#[test]
fn files_neither_tag() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file_1, tag_file_2, test_tags_1, test_tags_2) =
        super::two_files_multiple_tags_prepare(&temp_dir);

    let mut db = Database::new(&DatabaseMode::ReadWrite, &db_path);
    commands::tag::tag_file(&mut db, &tag_file_1, &test_tags_1, false).unwrap();
    commands::tag::tag_file(&mut db, &tag_file_2, &test_tags_2, false).unwrap();

    let db = Database::new(&DatabaseMode::Read, &db_path);
    assert_eq!(
        "",
        commands::files::get_file_paths(&db, &[&test_tags_1[..], &test_tags_2[..]].concat())
            .unwrap()
    );
}

#[test]
fn delete_file_tag() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file, _, test_tags, _) = super::two_files_multiple_tags_prepare(&temp_dir);

    let mut db = Database::new(&DatabaseMode::ReadWrite, &db_path);
    commands::tag::tag_file(&mut db, &tag_file, &test_tags, false).unwrap();
    let db = Database::new(&DatabaseMode::Read, &db_path);
    let tags = commands::tags::get_file_tags(&db, &tag_file).unwrap();
    assert_eq!(test_tags.join(","), tags);

    let mut db = Database::new(&DatabaseMode::ReadWrite, &db_path);
    commands::tag::tag_file(&mut db, &tag_file, &test_tags, true).unwrap();
    let db = Database::new(&DatabaseMode::Read, &db_path);
    let tags = commands::tags::get_file_tags(&db, &tag_file).unwrap();
    assert_eq!("", tags);
}

#[test]
fn files_clean_after_unreference() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file, _, test_tags, _) = super::two_files_multiple_tags_prepare(&temp_dir);

    let mut db = Database::new(&DatabaseMode::ReadWrite, &db_path);
    commands::tag::tag_file(&mut db, &tag_file, &test_tags, false).unwrap();
    let db = Database::new(&DatabaseMode::Read, &db_path);
    let tags = commands::tags::get_file_tags(&db, &tag_file).unwrap();
    assert_eq!(test_tags.join(","), tags);

    let mut db = Database::new(&DatabaseMode::ReadWrite, &db_path);
    commands::tag::tag_file(&mut db, &tag_file, &test_tags, true).unwrap();
    let db = Database::new(&DatabaseMode::Read, &db_path);
    let tags = commands::tags::get_file_tags(&db, &tag_file).unwrap();
    assert_eq!("", tags);
}
