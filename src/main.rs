mod db;
#[cfg(test)]
mod tests;

use crate::db::Database;
use clap::{Parser, builder::NonEmptyStringValueParser};
use rusqlite::Connection;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = Path::new(".anytaggy.sql").to_path_buf().into_os_string())]
    database_path: PathBuf,

    file: PathBuf,

    #[arg(short, long, value_parser = NonEmptyStringValueParser::new(), value_delimiter=',')]
    tags: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let parse = Args::parse();
    #[cfg(test)]
    let conn = Connection::open_in_memory()?;
    #[cfg(not(test))]
    let conn = Connection::open(parse.database_path)?;

    let mut db = Database::new(conn);
    db.apply_migrations();

    Ok(())
}
