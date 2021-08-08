mod bench;
mod test;

use std::path::PathBuf;

use bench::{run_bench, run_benches};
use cfg_expr::targets::TargetInfo;
use test::{run_test, run_tests};
use tracing::debug;

use crate::TaiResult;

#[derive(Debug)]
pub enum Mode {
    Bench,
    Test,
    Benches,
    Tests,
}

#[derive(Debug)]
pub struct Options {
    pub general: GeneralOptions,

    pub platform: PlatformOptions,
}

#[derive(Debug)]
pub struct GeneralOptions {
    pub mode: Mode,
    pub compiler: CompilerOptions,
    pub binary: BinaryOptions,
}

#[derive(Debug)]
pub struct CompilerOptions {
    /// Build for the target triples
    pub target: TargetInfo<'static>,

    // cargo arguments
    pub cargo_args: Vec<String>,
}

#[derive(Debug)]
pub struct BinaryOptions {
    pub args: Option<Vec<String>>,
    pub envs: Option<Vec<(String, String)>>,
    pub resources: Option<Vec<(String, PathBuf)>>,
}

#[derive(Debug)]
pub struct PlatformOptions {
    pub android_api_lvl: Option<u8>,
    pub android_ndk: Option<PathBuf>,
    pub ios_mobile_provision: Option<PathBuf>,
}

pub fn run_mode(requested: Options) -> TaiResult<()> {
    debug!("run with options:\n{:?}", requested);
    match requested.general.mode {
        Mode::Bench => run_bench(requested),
        Mode::Test => run_test(requested),
        Mode::Benches => run_benches(requested),
        Mode::Tests => run_tests(requested),
    }
}
