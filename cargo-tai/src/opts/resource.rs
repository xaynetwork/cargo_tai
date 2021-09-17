use std::path::PathBuf;

use structopt::StructOpt;

use super::parse_key_val;

#[derive(StructOpt, Debug)]
pub struct ResourceOptions {
    /// Resources to include in the app. Format: `id=local_path`
    ///
    /// Example:
    ///
    /// `cargo-tai test -r test_txt=./data/text.txt`
    #[structopt(short, long, parse(try_from_str = parse_key_val))]
    pub resources: Option<Vec<(String, PathBuf)>>,
}
