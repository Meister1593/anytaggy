use crate::{
    commands,
    db::{Database, DatabaseMode},
};
use temp_dir::TempDir;

#[test]
fn tag_file() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file, _, test_tags, _) = super::two_files_multiple_tags_prepare(&temp_dir);

    let mut db = Database::new(&DatabaseMode::ReadWrite, &db_path);
    commands::tag::tag_file(&mut db, &tag_file, &test_tags, false).unwrap();

    let db: Database = Database::new(&DatabaseMode::Read, &db_path);
    let tags = commands::tags::get_file_tags(&db, &tag_file).unwrap();
    assert_eq!(test_tags.join(","), tags);
}
