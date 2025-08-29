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
    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long, default_value = Path::new(".anytaggy.db").to_path_buf().into_os_string())]
    database_path: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Tag {
        file_path: PathBuf,

        #[arg(short, long, value_parser = NonEmptyStringValueParser::new(), value_delimiter=',')]
        tags: Vec<String>,

        #[arg(short, long)]
        delete: bool,
    },
    Tags {
        file_path: Option<PathBuf>,
    },

    Files {
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
        Command::Tag {
            file_path,
            tags,
            delete,
        } => {
            if tags.is_empty(){
                println!("ERROR: No tags specified");

                return Ok(ExitCode::FAILURE);
            }

            let mut db = Database::new(&DatabaseMode::ReadWrite, &parse.database_path);

            commands::tag::tag_file(&mut db, &file_path, &tags, delete)?;

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
