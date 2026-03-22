use crate::{AppError, db::Database};

pub fn repair(_db: &mut Database) -> Result<(), AppError> {
    Ok(())
}
