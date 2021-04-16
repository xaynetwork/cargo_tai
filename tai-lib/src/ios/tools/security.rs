use std::{
    path::Path,
    process::{Command, Output},
};

use anyhow::anyhow;

use crate::TaiResult;

const SECURITY: &str = "security";

pub fn find_identities() -> TaiResult<Output> {
    Command::new(SECURITY)
        .args(&["find-identity", "-v", "-p", "codesigning"])
        .output()
        .map_err(|err| anyhow!("{}", err))
}

pub fn find_certificate(name: &str) -> TaiResult<Output> {
    Command::new(SECURITY)
        .args(&["find-certificate", "-a", "-c", name, "-p"])
        .output()
        .map_err(|err| anyhow!("{}", err))
}

pub fn decode_cms<P: AsRef<Path>>(file: P) -> TaiResult<Output> {
    Command::new(SECURITY)
        .args(&["cms", "-D", "-i"])
        .arg(file.as_ref())
        .output()
        .map_err(|err| anyhow!("{}", err))
}
