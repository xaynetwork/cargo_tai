use std::{
    path::Path,
    process::{Command, Output},
};

use anyhow::anyhow;

use crate::TaiResult;

const SECURITY: &str = "security";

pub fn decode_cms<P: AsRef<Path>>(file: P) -> TaiResult<Output> {
    Command::new(SECURITY)
        .args(&["cms", "-D", "-i"])
        .arg(file.as_ref())
        .output()
        .map_err(|err| anyhow!("{}", err))
}
