use crate::db::{
    Database, DatabaseError, DbFile, File,
    tables::{
        file_tags::{get_tag_ids_by_file_id, unreference_file_tag},
        tags::get_tag_by_name,
    },
};
use rusqlite::{Connection, OptionalExtension};
use tracing::debug;

impl Database {
    pub fn get_all_files_paths(&self) -> Result<Vec<String>, DatabaseError> {
        get_all_files_paths(&self.connection).map_err(DatabaseError::DatabaseInternal)
    }

    pub fn get_all_files(&self) -> Result<Vec<DbFile>, DatabaseError> {
        get_all_files(&self.connection).map_err(DatabaseError::DatabaseInternal)
    }

    pub fn get_files_by_contents_hash(
        &self,
        contents_hash: &str,
    ) -> Result<Vec<DbFile>, DatabaseError> {
        get_file_by_contents_hash(&self.connection, contents_hash)
            .map_err(DatabaseError::DatabaseInternal)
    }

    pub fn update_file(&mut self, file: DbFile) -> Result<(), DatabaseError> {
        let tx = self.connection.transaction()?;

        update_file(&tx, &file).map_err(DatabaseError::DatabaseInternal)?;

        tx.commit()?;

        Ok(())
    }

    pub fn bulk_update(&mut self, files: &[DbFile]) -> Result<(), DatabaseError> {
        let tx = self.connection.transaction()?;

        for file in files {
            update_file(&tx, file).map_err(DatabaseError::DatabaseInternal)?;
        }

        tx.commit()?;

        Ok(())
    }

    pub fn bulk_update_and_delete(
        &mut self,
        files_to_update: &[DbFile],
        files_to_delete: &[DbFile],
    ) -> Result<(), DatabaseError> {
        let mut tx = self.connection.transaction()?;

        {
            let sp = tx.savepoint()?;
            for file in files_to_update {
                update_file(&sp, file).map_err(DatabaseError::DatabaseInternal)?;
            }
            sp.commit()?;
        }
        {
            let sp = tx.savepoint()?;
            for file in files_to_delete {
                delete_file(&sp, file.id).map_err(DatabaseError::DatabaseInternal)?;
            }
            sp.commit()?;
        }

        tx.commit()?;

        Ok(())
    }

    pub fn untag_file(&mut self, file: &File, tag_names: &[&str]) -> Result<(), DatabaseError> {
        let tx = self.connection.transaction()?;

        let Some(file_id) = get_file_id_by_fingerprint_hash(&tx, &file.fingerprint_hash)? else {
            return Err(DatabaseError::NoSuchFile);
        };
        debug!("found file_id {file_id}");

        let mut unreferenced_tags_count = 0;
        let file_tag_ids = get_tag_ids_by_file_id(&tx, file_id)?;
        for tag_name in tag_names {
            let Some(tag) = get_tag_by_name(&tx, tag_name)? else {
                return Err(DatabaseError::NoSuchTag((*tag_name).into()));
            };
            debug!("found tag_id {}", tag.id);

            if file_tag_ids.contains(&tag.id) {
                unreference_file_tag(&tx, file_id, tag.id)?;
                unreferenced_tags_count += 1;
            } else {
                return Err(DatabaseError::NoSuchTagOnFile(tag.name));
            }
        }

        // if we deleted all tags from file
        if file_tag_ids.len() == unreferenced_tags_count {
            // delete the file from database as unnecessary
            delete_file(&tx, file_id)?;
        }

        tx.commit()?;

        Ok(())
    }
}

pub fn delete_file(conn: &Connection, id: i32) -> Result<(), rusqlite::Error> {
    conn.execute(
        "DELETE FROM files
             WHERE id = ?1",
        (id,),
    )?;
    debug!("deleted file with id: {id}");

    Ok(())
}

pub fn create_file(conn: &Connection, file: &File) -> Result<DbFile, rusqlite::Error> {
    let mut insert = conn.prepare(
        "INSERT INTO files (path, name, contents_hash, fingerprint_hash, size) 
             VALUES (?1, ?2, ?3, ?4, ?5) 
             RETURNING id, path, name, contents_hash, fingerprint_hash, size",
    )?;

    let db_file = insert.query_one(
        (
            &file.path,
            &file.name,
            &file.contents_hash,
            &file.fingerprint_hash,
            &file.size,
        ),
        |row| {
            Ok(DbFile::builder()
                .id(row.get(0)?)
                .path(row.get(1)?)
                .name(row.get(2)?)
                .contents_hash(row.get(3)?)
                .fingerprint_hash(row.get(4)?)
                .size(row.get(5)?)
                .build())
        },
    )?;
    debug!("created file {file:?}");

    Ok(db_file)
}

pub fn update_file(conn: &Connection, file: &DbFile) -> Result<(), rusqlite::Error> {
    conn.execute(
        "UPDATE files 
             SET path = ?2, name = ?3, contents_hash = ?4, fingerprint_hash = ?5
             WHERE id = ?1",
        (
            &file.id,
            &file.path,
            &file.name,
            &file.contents_hash,
            &file.fingerprint_hash,
        ),
    )?;

    debug!("updated file {file:?}");
    Ok(())
}

pub fn get_file_id_by_fingerprint_hash(
    conn: &Connection,
    fingerprint_hash: &str,
) -> Result<Option<i32>, rusqlite::Error> {
    let mut select = conn.prepare(
        "SELECT id 
            FROM files 
            WHERE fingerprint_hash = ?1",
    )?;

    select
        .query_one([&fingerprint_hash], |row| row.get(0))
        .optional()
}

// todo: technically there could be a hash collision, this needs double checking
pub fn get_file_by_contents_hash(
    conn: &Connection,
    contents_hash: &str,
) -> Result<Vec<DbFile>, rusqlite::Error> {
    let mut query = conn.prepare(
        "SELECT *
            FROM files 
            WHERE contents_hash = ?1",
    )?;

    Ok(query
        .query_map([&contents_hash], |row| {
            Ok(DbFile::builder()
                .id(row.get(0)?)
                .path(row.get(1)?)
                .name(row.get(2)?)
                .contents_hash(row.get(3)?)
                .fingerprint_hash(row.get(4)?)
                .size(row.get(5)?)
                .build())
        })?
        .filter_map(Result::ok)
        .collect())
}

fn get_all_files_paths(conn: &Connection) -> Result<Vec<String>, rusqlite::Error> {
    let mut query = conn.prepare(
        "SELECT path 
            FROM files",
    )?;

    Ok(query
        .query_map([], |row| row.get(0))?
        .filter_map(Result::ok)
        .collect())
}

fn get_all_files(conn: &Connection) -> Result<Vec<DbFile>, rusqlite::Error> {
    let mut query = conn.prepare(
        "SELECT *
            FROM files",
    )?;

    Ok(query
        .query_map([], |row| {
            Ok(DbFile::builder()
                .id(row.get(0)?)
                .path(row.get(1)?)
                .name(row.get(2)?)
                .contents_hash(row.get(3)?)
                .fingerprint_hash(row.get(4)?)
                .size(row.get(5)?)
                .build())
        })?
        .filter_map(Result::ok)
        .collect())
}
