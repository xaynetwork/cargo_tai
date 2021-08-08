use std::process::Command;

use crate::TaiResult;

pub fn benches_command() -> TaiResult<Command> {
    let mut cmd = Command::new("cargo");
    cmd.args(&["build", "--release", "--benches"]);
    Ok(cmd)
}

pub fn tests_command() -> TaiResult<Command> {
    let mut cmd = Command::new("cargo");
    cmd.args(&["build", "--tests"]);
    Ok(cmd)
}

pub fn bench_command() -> TaiResult<Command> {
    let mut cmd = Command::new("cargo");
    cmd.args(&["build", "--release", "--bench"]);
    Ok(cmd)
}

pub fn test_command() -> TaiResult<Command> {
    let mut cmd = Command::new("cargo");
    cmd.args(&["build", "--test"]);
    Ok(cmd)
}
