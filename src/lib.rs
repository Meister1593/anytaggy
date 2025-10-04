mod commands;
pub mod db;

use crate::db::{Database, DatabaseMode};
use clap::{Parser, Subcommand, builder::NonEmptyStringValueParser};
use std::path::{Path, PathBuf};
use thiserror::Error;
use tracing::{debug, error};

pub const DATABASE_FILENAME: &str = ".anytaggy.db";

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Specified database file could not be found")]
    DatabaseNotFound,
    #[error("Couldn't retrieve file name from path")]
    CantGetFileNameFromPath,
    #[error("No tags specified")]
    NoTagsSpecified,
    #[error("Could not access file outside of database structure")]
    FileOutsideStructure,
    #[error("Database error: {0}")]
    Database(#[from] db::DatabaseError),
    #[error("Unhandled error: {0}")]
    Unhandled(#[from] std::io::Error),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, help = format!("Path to database to store tags, files metadata. Defaults to '{DATABASE_FILENAME}'"))]
    pub database_path: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Tagging files with tag names.
    /// Creates database, tags inside database if required
    Tag {
        /// Path to the file
        file_path: PathBuf,

        /// Tags to add to the file
        #[arg(short, long, value_parser = NonEmptyStringValueParser::new(), value_delimiter=',')]
        tags: Vec<String>,
    },
    /// Untag files from database.
    /// Does not delete tags, only un-references files from them
    Untag {
        /// Path to the file with tags
        file_path: PathBuf,

        /// Tags to remove from file
        #[arg(short, long, value_parser = NonEmptyStringValueParser::new(), value_delimiter=',')]
        tags: Vec<String>,
    },
    /// Delete tags.
    /// Will also remove tags from existing files in database
    RmTags {
        /// Tags to remove
        #[arg(value_parser = NonEmptyStringValueParser::new(), value_delimiter=',')]
        tags: Vec<String>,
    },
    /// List tags
    Tags {
        /// Path to the file with tags.
        /// If not specified, lists all tags from database
        file_path: Option<PathBuf>,
    },
    /// List files
    Files {
        /// Tags to list files with.
        /// If not specified, lists all files from database
        #[arg(value_parser = NonEmptyStringValueParser::new(), value_delimiter=' ')]
        tags: Option<Vec<String>>,
    },
}

#[allow(clippy::missing_errors_doc)]
#[allow(clippy::too_many_lines)]
pub fn entrypoint(args: Args) -> Result<Option<String>, AppError> {
    let is_tag_subcommand = matches!(
        args.command,
        Command::Tag {
            file_path: _,
            tags: _
        }
    );
    let database_path = if let Some(database_path) = args.database_path {
        // If database path was specified, and it is not a tag subcommand (can't create new database)
        // Then error out as user error
        if !database_path.exists() && !is_tag_subcommand {
            return Err(AppError::DatabaseNotFound);
        }

        database_path
    } else if let Some(database_path) = search_database_in_parent_folders() {
        // Search database path from current and parent folders
        database_path
    } else if is_tag_subcommand {
        // If it's a root and we still couldn't find database, check if it's a tag subcommand
        //  and if true, assume initial path to be the right one (new database will be created)
        std::env::current_dir()?.join(DATABASE_FILENAME)
    } else {
        // If it's not found and database will not be created - error out
        return Err(AppError::DatabaseNotFound);
    };
    debug!("database_path: {}", database_path.display());

    match args.command {
        Command::Tag { file_path, tags } => {
            if tags.is_empty() {
                return Err(AppError::NoTagsSpecified);
            }

            let mut db = Database::new(&DatabaseMode::ReadWriteCreate, &database_path);

            if !check_file_paths_for_subdirectory(&database_path, &file_path)? {
                return Err(AppError::FileOutsideStructure);
            }

            commands::tag::tag_file(
                &mut db,
                &file_path,
                &tags.iter().map(String::as_str).collect::<Vec<_>>(),
            )
            .map(|()| None)
        }
        Command::Untag { file_path, tags } => {
            if tags.is_empty() {
                return Err(AppError::NoTagsSpecified);
            }

            let mut db = Database::new(&DatabaseMode::ReadWrite, &database_path);

            if !check_file_paths_for_subdirectory(&database_path, &file_path)? {
                return Err(AppError::FileOutsideStructure);
            }

            commands::untag::untag_file(
                &mut db,
                &file_path,
                &tags.iter().map(String::as_str).collect::<Vec<_>>(),
            )
            .map(|()| None)
        }
        Command::Tags { file_path } => {
            let db = Database::new(&DatabaseMode::Read, &database_path);

            if let Some(file_path) = file_path {
                if !check_file_paths_for_subdirectory(&database_path, &file_path)? {
                    return Err(AppError::FileOutsideStructure);
                }

                commands::tags::get_file_tags(&db, &file_path)
            } else {
                commands::tags::get_all_tags(&db)
            }
        }
        Command::RmTags { tags } => {
            if tags.is_empty() {
                return Err(AppError::NoTagsSpecified);
            }

            let mut db = Database::new(&DatabaseMode::ReadWrite, &database_path);

            commands::rm_tags::rm_tags(
                &mut db,
                &tags.iter().map(String::as_str).collect::<Vec<_>>(),
            )
            .map(|()| None)
        }
        Command::Files { tags } => {
            let db = Database::new(&DatabaseMode::Read, &database_path);

            if let Some(tags) = tags {
                if tags.is_empty() {
                    Err(AppError::NoTagsSpecified)
                } else {
                    commands::files::get_file_paths(
                        &db,
                        &tags.iter().map(String::as_str).collect::<Vec<_>>(),
                    )
                }
            } else {
                commands::files::get_files(&db)
            }
        }
    }
}

fn check_file_paths_for_subdirectory(parent: &Path, child: &Path) -> Result<bool, AppError> {
    let parent = parent.canonicalize()?;
    debug!("parent_cannonical_path: {}", parent.display());

    let parent_dir = parent.parent().ok_or(AppError::DatabaseNotFound)?;
    debug!("parent_path: {}", parent_dir.display());

    let child = child.canonicalize()?;
    debug!("child_cannonical_path: {}", child.display());

    Ok(child.starts_with(parent_dir))
}

fn search_database_in_parent_folders() -> Option<PathBuf> {
    match lets_find_up::find_up(DATABASE_FILENAME) {
        Ok(res) => res,
        Err(e) => {
            error!("{e:?}");
            None
        }
    }
}
