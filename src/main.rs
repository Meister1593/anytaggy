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
pub struct Args {
    /// Absolute path to database to store tags, files metadata
    #[arg(short, long, default_value = Path::new(".anytaggy.db").to_path_buf().into_os_string())]
    database_path: PathBuf,

    #[command(subcommand)]
    command: Command,
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
pub fn entrypoint(args: Args) -> anyhow::Result<(Option<String>, ExitCode)> {
    match args.command {
        Command::Tag { file_path, tags } => {
            if tags.is_empty() {
                return Ok((Some("ERROR: No tags specified".into()), ExitCode::FAILURE));
            }

            let mut db = Database::new(&DatabaseMode::ReadWrite, &args.database_path);

            commands::tag::tag_file(&mut db, &file_path, &tags)?;

            Ok((None, ExitCode::SUCCESS))
        }
        Command::Untag { file_path, tags } => {
            if !args.database_path.exists() {
                return Ok((
                    Some("ERROR: Database file could not be found".into()),
                    ExitCode::FAILURE,
                ));
            }

            if tags.is_empty() {
                return Ok((Some("ERROR: No tags specified".into()), ExitCode::FAILURE));
            }

            let mut db = Database::new(&DatabaseMode::ReadWrite, &args.database_path);

            let result = commands::untag::untag_file(&mut db, &file_path, &tags);
            match result {
                Ok(()) => Ok((None, ExitCode::SUCCESS)),
                Err(err) => Ok((Some(err.to_string()), ExitCode::FAILURE)),
            }
        }
        Command::Tags { file_path } => {
            if !args.database_path.exists() {
                return Ok((
                    Some("ERROR: Database file could not be found".into()),
                    ExitCode::FAILURE,
                ));
            }

            let db = Database::new(&DatabaseMode::Read, &args.database_path);

            let result = if let Some(file_path) = file_path {
                commands::tags::get_file_tags(&db, &file_path)
            } else {
                commands::tags::get_all_tags(&db)
            };
            match result {
                Ok(out) => Ok((Some(out), ExitCode::SUCCESS)),
                Err(err) => Ok((Some(err.to_string()), ExitCode::FAILURE)),
            }
        }
        Command::RmTags { tags } => {
            if !args.database_path.exists() {
                return Ok((
                    Some("ERROR: Database file could not be found".into()),
                    ExitCode::FAILURE,
                ));
            }

            if tags.is_empty() {
                return Ok((Some("ERROR: No tags specified".into()), ExitCode::FAILURE));
            }

            let mut db = Database::new(&DatabaseMode::ReadWrite, &args.database_path);

            let result = commands::rm_tags::rm_tags(&mut db, &tags);

            match result {
                Ok(()) => Ok((None, ExitCode::SUCCESS)),
                Err(err) => Ok((Some(err.to_string()), ExitCode::FAILURE)),
            }
        }
        Command::Files { tags } => {
            if !args.database_path.exists() {
                return Ok((
                    Some("ERROR: Database file could not be found".into()),
                    ExitCode::FAILURE,
                ));
            }

            let db = Database::new(&DatabaseMode::Read, &args.database_path);

            let result = if let Some(tags) = tags {
                commands::files::get_file_paths(&db, &tags)
            } else {
                commands::files::get_files(&db)
            };
            match result {
                Ok(out) => Ok((Some(out), ExitCode::SUCCESS)),
                Err(err) => Ok((Some(err.to_string()), ExitCode::FAILURE)),
            }
        }
    }
}

fn main() -> anyhow::Result<ExitCode> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
    let (out, exit_code) = entrypoint(Args::parse())?;
    if let Some(out) = out {
        println!("{out}");
    }

    Ok(exit_code)
}
