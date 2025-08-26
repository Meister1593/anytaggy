#![warn(clippy::pedantic)]
mod commands;
mod db;
#[cfg(test)]
mod tests;

use crate::db::Database;
use clap::{Parser, Subcommand, builder::NonEmptyStringValueParser};
use rusqlite::Connection;
use std::path::{Path, PathBuf};
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
    },
    Tags {
        file_path: PathBuf,
    },

    Files {
        #[arg(value_parser = NonEmptyStringValueParser::new(), value_delimiter=' ')]
        tags: Vec<String>,
    },
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let parse = Args::parse();
    let conn = Connection::open(parse.database_path)?;

    let mut db = Database::new(conn);

    match parse.command {
        Command::Tag { file_path, tags } => commands::tag::tag(&mut db, &file_path, tags),
        Command::Tags { file_path } => {
            println!("{}", commands::tags::tags(&db, &file_path)?);
            Ok(())
        }
        Command::Files { tags } => {
            println!("{}", commands::files::files(&db, tags)?);
            Ok(())
        }
    }?;

    Ok(())
}
