mod bench;
mod test;

use bench::run_benches;
use cfg_expr::targets::TargetInfo;
use test::run_tests;
use tracing::debug;

use crate::TaiResult;

#[derive(Debug)]
pub enum Mode {
    Test,
    Bench,
}

#[derive(Debug)]
pub struct Options {
    // cargo
    /// Build artifacts in release mode, with optimizations
    pub release: bool,

    /// Build for the target triples
    pub target: TargetInfo<'static>,

    /// Activate all available features
    pub all_features: bool,

    /// Do not activate the `default` feature
    pub no_default_features: bool,

    /// Space-separated list of features to activate
    pub features: Vec<String>,

    pub mode: Mode,

    // android
    pub android_platform: u8,

    // application
    pub envs: Option<Vec<(String, String)>>,
}

pub fn run_mode(requested: &Options) -> TaiResult<()> {
    debug!("run with options:\n{:?}", requested);
    match requested.mode {
        Mode::Test => run_tests(requested),
        Mode::Bench => run_benches(requested),
    }
}
