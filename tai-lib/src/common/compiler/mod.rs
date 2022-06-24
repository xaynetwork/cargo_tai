use cfg_expr::targets::TargetInfo;
use std::{path::PathBuf, process::Command};

mod util;

use util::{compile, is_bench, is_test};

#[derive(Debug)]
pub struct BuiltUnit {
    pub name: String,
    pub artifact: PathBuf,
    pub target: TargetInfo<'static>,
}

use crate::{common::opts::CompilerOptions, TaiResult};

pub fn compile_tests(cmd: Command, requested: &CompilerOptions) -> TaiResult<Vec<BuiltUnit>> {
    compile(cmd, requested, is_test)
}

pub fn compile_benches(cmd: Command, requested: &CompilerOptions) -> TaiResult<Vec<BuiltUnit>> {
    compile(cmd, requested, is_bench)
}
