use temp_dir::TempDir;

use crate::{
    commands,
    db::{Database, DatabaseMode},
};

#[test]
fn delete_file_tag() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file, _, test_tags, _) = super::two_files_multiple_tags_prepare(&temp_dir);

    let mut db = Database::new(&DatabaseMode::ReadWrite, &db_path);
    commands::tag::tag_file(&mut db, &tag_file, &test_tags).unwrap();
    let db = Database::new(&DatabaseMode::Read, &db_path);
    let tags = commands::tags::get_file_tags(&db, &tag_file).unwrap();
    assert_eq!(test_tags.join(","), tags);

    let mut db = Database::new(&DatabaseMode::ReadWrite, &db_path);
    commands::untag::untag_file(&mut db, &tag_file, &test_tags).unwrap();
    let db = Database::new(&DatabaseMode::Read, &db_path);
    let tags = commands::tags::get_file_tags(&db, &tag_file).unwrap();
    assert_eq!("", tags);
}

#[test]
fn files_clean_after_delete_untag() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file, _, test_tags, _) = super::two_files_multiple_tags_prepare(&temp_dir);

    let mut db = Database::new(&DatabaseMode::ReadWrite, &db_path);
    commands::tag::tag_file(&mut db, &tag_file, &test_tags).unwrap();
    let db = Database::new(&DatabaseMode::Read, &db_path);
    let tags = commands::tags::get_file_tags(&db, &tag_file).unwrap();
    assert_eq!(test_tags.join(","), tags);

    let mut db = Database::new(&DatabaseMode::ReadWrite, &db_path);
    commands::untag::untag_file(&mut db, &tag_file, &test_tags).unwrap();
    let db = Database::new(&DatabaseMode::Read, &db_path);
    let files = commands::files::get_files(&db).unwrap();
    assert_eq!("", files);
}
