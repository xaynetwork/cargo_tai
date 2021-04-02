use anyhow::Error;
use structopt::StructOpt;
use tai_lib::task::run_mode;

mod cli;

use cli::Options;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

fn main() -> Result<(), Error> {
    FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_ansi(true)
        .with_target(false)
        .init();

    let opt = Options::from_args();

    run_mode(&opt.into())
}
