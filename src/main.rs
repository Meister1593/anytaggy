#![warn(clippy::pedantic)]
use std::process::ExitCode;

use anytaggy::{Args, entrypoint};
use clap::Parser;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

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
