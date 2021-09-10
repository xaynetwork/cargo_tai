use std::{
    path::Path,
    process::{Command, Output},
};

use anyhow::anyhow;

use crate::TaiResult;

const XCODEGEN: &str = "xcodegen";

pub fn generate<S: AsRef<Path>, P: AsRef<Path>>(spec: S, project: P) -> TaiResult<Output> {
    Command::new(XCODEGEN)
        .args(&["generate", "--spec"])
        .arg(spec.as_ref())
        .arg("--project")
        .arg(project.as_ref())
        .output()
        .map_err(|err| anyhow!("{}", err))
}
