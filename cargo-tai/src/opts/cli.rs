use clap::Parser;
use tai_lib::common::opts;

#[derive(Parser, Debug)]
pub struct CliOptions {
    #[clap(short, long)]
    pub verbose: bool,
}

impl From<CliOptions> for opts::CliOptions {
    fn from(CliOptions { verbose }: CliOptions) -> Self {
        opts::CliOptions { verbose }
    }
}
