mod commands;
mod db;
#[cfg(test)]
mod tests;

use crate::db::Database;
use clap::{Parser, Subcommand, builder::NonEmptyStringValueParser};
use rusqlite::Connection;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = Path::new(".anytaggy.db").to_path_buf().into_os_string())]
    database_path: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Tag {
        file: PathBuf,

        #[arg(short, long, value_parser = NonEmptyStringValueParser::new(), value_delimiter=',')]
        tags: Vec<String>,
    },
    Tags {
        file: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let parse = Args::parse();
    let conn = Connection::open(parse.database_path)?;

    let mut db = Database::new(conn);
    db.apply_migrations();

    match parse.command {
        Command::Tag { file, tags } => commands::tag::tag(db, file, tags),
        Command::Tags { file } => commands::tags::tags(db, file),
    }?;

    Ok(())
}
