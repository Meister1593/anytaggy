use crate::{
    commands,
    db::{Database, DatabaseMode},
};
use temp_dir::TempDir;

#[test]
fn get_tags() {
    // Test data
    let temp_dir = TempDir::new().unwrap();
    let (db_path, tag_file_1, _, test_tags_1, _) =
        super::two_files_multiple_tags_prepare(&temp_dir);

    let mut db = Database::new(&DatabaseMode::ReadWrite, &db_path);
    commands::tag::tag_file(&mut db, &tag_file_1, &test_tags_1, false).unwrap();

    let db = Database::new(&DatabaseMode::Read, &db_path);
    assert_eq!(
        test_tags_1.join(","),
        commands::tags::get_all_tags(&db).unwrap()
    );
}
