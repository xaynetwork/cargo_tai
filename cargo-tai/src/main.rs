use clap::Parser;
use tai_lib::common::{command::run, opts::Options};

mod opts;

use opts::Command;
use tracing_subscriber::{prelude::*, EnvFilter};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_ansi(true)
        .with_target(false)
        .without_time()
        .finish()
        .init();

    let command = Command::from_args();
    let requested: Options = command.into();

    let _ = run(requested);
}
