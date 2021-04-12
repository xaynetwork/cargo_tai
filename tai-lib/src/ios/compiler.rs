use std::process::Command;

use crate::TaiResult;

pub fn test_command() -> TaiResult<Command> {
    let mut cmd = Command::new("cargo");
    cmd.args(&["build", "--tests"]);
    Ok(cmd)
}

pub fn bench_command() -> TaiResult<Command> {
    let mut cmd = Command::new("cargo");
    cmd.args(&["build", "--benches"]);
    Ok(cmd)
}