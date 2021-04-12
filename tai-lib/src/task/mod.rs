mod bench;
mod test;

use std::path::PathBuf;

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
    pub mode: Mode,

    /// Build for the target triples
    pub target: TargetInfo<'static>,

    // android
    pub android_platform: u8,

    // application
    pub args: Option<Vec<String>>,
    pub envs: Option<Vec<(String, String)>>,
    pub resources: Option<Vec<(String, PathBuf)>>,

    // cargo arguments
    pub cargo_args: Vec<String>,
}

pub fn run_mode(requested: &Options) -> TaiResult<()> {
    debug!("run with options:\n{:?}", requested);
    match requested.mode {
        Mode::Test => run_tests(requested),
        Mode::Bench => run_benches(requested),
    }
}