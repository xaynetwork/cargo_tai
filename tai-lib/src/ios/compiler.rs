use std::process::Command;

use crate::{common::opts::CompilerOptions, TaiResult};

pub fn benches_command(requested: &CompilerOptions) -> TaiResult<Command> {
    let mut cmd = Command::new("cargo");
    cmd.args(["build", "--benches", "--target", requested.target.triple]);
    Ok(cmd)
}

pub fn tests_command(requested: &CompilerOptions) -> TaiResult<Command> {
    let mut cmd = Command::new("cargo");
    cmd.args(["build", "--tests", "--target", requested.target.triple]);
    Ok(cmd)
}

pub fn bench_command(requested: &CompilerOptions) -> TaiResult<Command> {
    let mut cmd = Command::new("cargo");
    cmd.args(["build", "--bench", "--target", requested.target.triple]);
    Ok(cmd)
}

pub fn test_command(requested: &CompilerOptions) -> TaiResult<Command> {
    let mut cmd = Command::new("cargo");
    cmd.args(["build", "--test", "--target", requested.target.triple]);
    Ok(cmd)
}
