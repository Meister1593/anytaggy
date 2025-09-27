#![warn(clippy::pedantic)]
mod commands;
mod db;

use crate::db::{Database, DatabaseMode};
use anyhow::anyhow;
use clap::{Parser, Subcommand, builder::NonEmptyStringValueParser};
use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};
use tracing::debug;

pub const DATABASE_FILENAME: &str = ".anytaggy.db";

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

fn search_database_in_parent_folders(
    initial_path: &Path,
) -> std::option::Option<std::path::PathBuf> {
    debug!("initial search path: {}", initial_path.display());
    let mut database_path = initial_path.to_path_buf();
    // Traverse through parents until root
    while !database_path.exists() {
        // If currently looking path exists
        if let Some(parent) = database_path.parent()
            && let Some(parent_dir) = parent.parent()
        // (and it is a directory)
        {
            // Append to that path database filename, check on next iteration for existence
            database_path = parent_dir.join(DATABASE_FILENAME);
        } else {
            return None;
        }
        debug!("parent: {}", database_path.display());
    }
    Some(database_path)
}

#[allow(clippy::missing_errors_doc)]
pub fn entrypoint(args: Args) -> anyhow::Result<(Option<String>, ExitCode)> {
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
            return Ok((
                Some("ERROR: Specified database file could not be found".into()),
                ExitCode::FAILURE,
            ));
        }

        database_path
    } else {
        let initial_database_path = std::env::current_dir()?.join(DATABASE_FILENAME);
        if let Some(database_path) = search_database_in_parent_folders(&initial_database_path) {
            database_path
        } else {
            // If it's a root and we still couldn't find database, check if it's a tag subcommand
            if is_tag_subcommand {
                // If it is, then assume initial path to be the right one (new database will be created)
                initial_database_path
            } else {
                // If it's not found and database will not be created - error out
                return Ok((
                    Some("ERROR: Database file could not be found".into()),
                    ExitCode::FAILURE,
                ));
            }
        }
    };
    debug!("database_path: {}", database_path.display());

    let result = match args.command {
        Command::Tag { file_path, tags } => {
            if tags.is_empty() {
                return Ok((Some("ERROR: No tags specified".into()), ExitCode::FAILURE));
            }

            let mut db = Database::new(&DatabaseMode::ReadWriteCreate, &database_path);

            commands::tag::tag_file(&mut db, &file_path, &tags)?;

            Ok(None)
        }
        Command::Untag { file_path, tags } => {
            if tags.is_empty() {
                return Ok((Some("ERROR: No tags specified".into()), ExitCode::FAILURE));
            }

            let mut db = Database::new(&DatabaseMode::ReadWrite, &database_path);

            let result = commands::untag::untag_file(&mut db, &file_path, &tags);
            match result {
                Ok(()) => Ok(None),
                Err(err) => Err(err),
            }
        }
        Command::Tags { file_path } => {
            let db = Database::new(&DatabaseMode::Read, &database_path);

            if let Some(file_path) = file_path {
                commands::tags::get_file_tags(&db, &file_path)
            } else {
                commands::tags::get_all_tags(&db)
            }
        }
        Command::RmTags { tags } => {
            if tags.is_empty() {
                return Ok((Some("ERROR: No tags specified".into()), ExitCode::FAILURE));
            }

            let mut db = Database::new(&DatabaseMode::ReadWrite, &database_path);

            let result = commands::rm_tags::rm_tags(&mut db, &tags);
            match result {
                Ok(()) => Ok(None),
                Err(err) => Err(err),
            }
        }
        Command::Files { tags } => {
            let db = Database::new(&DatabaseMode::Read, &database_path);

            if let Some(tags) = tags {
                if tags.is_empty() {
                    Err(anyhow!("ERROR: No tags specified"))
                } else {
                    commands::files::get_file_paths(&db, &tags)
                }
            } else {
                commands::files::get_files(&db)
            }
        }
    };
    match result {
        Ok(out) => Ok((out, ExitCode::SUCCESS)),
        Err(err) => Ok((Some(err.to_string()), ExitCode::FAILURE)),
    }
}
