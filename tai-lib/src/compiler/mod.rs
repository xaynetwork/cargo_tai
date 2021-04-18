use cfg_expr::targets::TargetInfo;
use std::{path::PathBuf, process::Command};

mod util;

use util::{compile, is_bench, is_test};

#[derive(Debug)]
pub struct BuildUnit {
    pub name: String,
    pub executable: PathBuf,
    pub target: TargetInfo<'static>,
}

use crate::{task::CompilerOptions, TaiResult};

pub fn compile_tests(cmd: Command, requested: &CompilerOptions) -> TaiResult<Vec<BuildUnit>> {
    compile(cmd, requested, is_test)
}

pub fn compile_benches(cmd: Command, requested: &CompilerOptions) -> TaiResult<Vec<BuildUnit>> {
    compile(cmd, requested, is_bench)
}
