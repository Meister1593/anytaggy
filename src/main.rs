use std::path::PathBuf;

use clap::Parser;
use rusqlite::Connection;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file: PathBuf,

    #[arg(short, long)]
    tags: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let conn = Connection::open("amogus.sql")?;
    let parse = Args::parse();
    

    Ok(())
}
