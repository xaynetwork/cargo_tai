use anyhow::Error;

use clap::Parser;
use tai_lib::common::command::run_command;

mod opts;

use opts::Options;
use tracing_subscriber::{prelude::*, EnvFilter};

fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_ansi(true)
        .with_target(false)
        .without_time()
        .finish()
        .init();

    let opt = Options::from_args();
    let requested_opt: tai_lib::common::opts::Options = opt.into();

    run_command(requested_opt)
}
