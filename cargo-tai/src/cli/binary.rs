use std::path::PathBuf;

use structopt::StructOpt;
use tai_lib::common::options;

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

    /// Resources to include in the app. Format: `id=local_path`
    ///
    /// Example:
    ///
    /// `cargo-tai test -r test_txt=./data/text.txt`
    #[structopt(short, long, parse(try_from_str = parse_key_val))]
    pub resources: Option<Vec<(String, PathBuf)>>,
}

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn std::error::Error>>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + 'static,
    U: std::str::FromStr,
    U::Err: std::error::Error + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

impl From<BinaryOptions> for Option<options::BinaryOptions> {
    fn from(
        BinaryOptions {
            args,
            envs,
            resources,
        }: BinaryOptions,
    ) -> Self {
        match (&args, &envs, &resources) {
            (None, None, None) => None,
            _ => Some(options::BinaryOptions {
                args,
                envs,
                resources,
            }),
        }
    }
}
