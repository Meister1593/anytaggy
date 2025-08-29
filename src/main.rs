#![warn(clippy::pedantic)]
mod commands;
mod db;
#[cfg(test)]
mod tests;

use crate::db::{Database, DatabaseMode};
use clap::{Parser, Subcommand, builder::NonEmptyStringValueParser};
use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Absolute path to database to store tags, files metadata
    #[arg(short, long, default_value = Path::new(".anytaggy.db").to_path_buf().into_os_string())]
    database_path: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
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

fn main() -> anyhow::Result<ExitCode> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let parse = Args::parse();

    match parse.command {
        Command::Tag { file_path, tags } => {
            if tags.is_empty() {
                println!("ERROR: No tags specified");

                return Ok(ExitCode::FAILURE);
            }

            let mut db = Database::new(&DatabaseMode::ReadWrite, &parse.database_path);

            commands::tag::tag_file(&mut db, &file_path, &tags)?;

            Ok(ExitCode::SUCCESS)
        }
        Command::Untag { file_path, tags } => {
            if tags.is_empty() {
                println!("ERROR: No tags specified");

                return Ok(ExitCode::FAILURE);
            }

            let mut db = Database::new(&DatabaseMode::ReadWrite, &parse.database_path);

            commands::untag::untag_file(&mut db, &file_path, &tags)?;

            Ok(ExitCode::SUCCESS)
        }
        Command::Tags { file_path } => {
            if !parse.database_path.exists() {
                println!("ERROR: Database file could not be found");

                return Ok(ExitCode::FAILURE);
            }

            let db = Database::new(&DatabaseMode::Read, &parse.database_path);

            if let Some(file_path) = file_path {
                println!("{}", commands::tags::get_file_tags(&db, &file_path)?);
            } else {
                println!("{}", commands::tags::get_all_tags(&db)?);
            }

            Ok(ExitCode::SUCCESS)
        }
        Command::RmTags { tags } => {
            if tags.is_empty() {
                println!("ERROR: No tags specified");

                return Ok(ExitCode::FAILURE);
            }

            let mut db = Database::new(&DatabaseMode::ReadWrite, &parse.database_path);

            commands::rm_tags::rm_tags(&mut db, &tags)?;

            Ok(ExitCode::SUCCESS)
        }
        Command::Files { tags } => {
            if !parse.database_path.exists() {
                println!("ERROR: Database file could not be found");

                return Ok(ExitCode::FAILURE);
            }

            let db = Database::new(&DatabaseMode::Read, &parse.database_path);

            if let Some(tags) = tags {
                println!("{}", commands::files::get_file_paths(&db, &tags)?);
            } else {
                println!("{}", commands::files::get_files(&db)?);
            }

            Ok(ExitCode::SUCCESS)
        }
    }
}
