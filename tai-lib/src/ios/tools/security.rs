use std::{
    path::Path,
    process::{Command, Output},
};

use crate::TaiResult;

const SECURITY: &str = "security";

pub fn decode_cms<F: AsRef<Path>>(file: F) -> TaiResult<Output> {
    Command::new(SECURITY)
        .args(&["cms", "-D", "-i"])
        .arg(file.as_ref())
        .output()
        .map_err(Into::into)
}
