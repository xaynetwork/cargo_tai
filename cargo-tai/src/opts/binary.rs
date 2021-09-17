use structopt::StructOpt;
use tai_lib::common::opts;

use super::parse_key_val;

#[derive(StructOpt, Debug)]
pub struct BinaryOptions {
    /// A comma-separated list of arguments to pass to the app when launching it.
    ///
    /// Example:
    ///
    /// `cargo-tai test --args -Z,unstable-options,--report-time`
    #[structopt(short, long, allow_hyphen_values = true, use_delimiter = true)]
    pub args: Option<Vec<String>>,

    /// Environment variables to pass to the app when launching it. Format: `key=value`
    ///
    /// Example:
    ///
    /// `cargo-tai test --envs TAI_1=1 TAI_2=2`
    #[structopt(short, long, parse(try_from_str = parse_key_val))]
    pub envs: Option<Vec<(String, String)>>,
}

impl From<BinaryOptions> for Option<opts::BinaryOptions> {
    fn from(BinaryOptions { args, envs }: BinaryOptions) -> Self {
        Some(opts::BinaryOptions { args, envs })
    }
}
