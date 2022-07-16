use anyhow::Error;

use clap::Parser;
use tai_lib::common::command::run_command;

mod opts;

use opts::Options;
use tracing_subscriber::{fmt::format::FmtSpan, prelude::*, EnvFilter};

fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_ansi(true)
        .with_target(false)
        .with_level(false)
        .without_time()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .finish()
        .init();

    let opt = Options::from_args();
    let requested_opt: tai_lib::common::opts::Options = opt.into();

    run_command(requested_opt)
}
