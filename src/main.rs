#![warn(clippy::pedantic)]
use std::process::ExitCode;

use anytaggy::{Args, entrypoint};
use clap::Parser;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn main() -> std::process::ExitCode {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    match entrypoint(Args::parse()) {
        Ok(out) => {
            if let Some(out) = out {
                println!("{out}");
            }
            ExitCode::SUCCESS
        }
        Err(err) => {
            println!("{err}");

            ExitCode::FAILURE
        }
    }
}
