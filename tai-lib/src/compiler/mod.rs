mod test;
mod util;

use cfg_expr::targets::TargetInfo;
use std::path::PathBuf;

pub use test::{compile_bench, compile_tests};

#[derive(Debug)]
pub struct BuildUnit {
    pub name: String,
    pub executable: PathBuf,
    pub target: TargetInfo<'static>,
}
