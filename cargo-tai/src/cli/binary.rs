use structopt::StructOpt;
use tai_lib::common::options;

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

impl From<BinaryOptions> for Option<options::BinaryOptions> {
    fn from(BinaryOptions { args, envs }: BinaryOptions) -> Self {
        match (&args, &envs) {
            (None, None) => None,
            _ => Some(options::BinaryOptions { args, envs }),
        }
    }
}
