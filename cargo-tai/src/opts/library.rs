use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct LibraryOptions {
    /// Libraries to include in the app that the built units link to. Format: `local_path`
    ///
    /// Example:
    ///
    /// `cargo-tai test -l ./openssl-1.1.1s/libssl.so.1.1
    #[clap(short, long)]
    pub libraries: Option<Vec<PathBuf>>,
}
