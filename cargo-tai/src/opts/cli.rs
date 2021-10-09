use structopt::StructOpt;
use tai_lib::common::opts;

#[derive(StructOpt, Debug)]
pub struct CliOptions {
    #[structopt(short, long)]
    pub verbose: bool,
}

impl From<CliOptions> for opts::CliOptions {
    fn from(CliOptions { verbose }: CliOptions) -> Self {
        opts::CliOptions { verbose }
    }
}
