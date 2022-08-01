use std::{
    path::Path,
    process::{Command, Output},
};

use crate::TaiResult;

const LLDB: &str = "lldb";

pub fn run_source<S: AsRef<Path>>(source: S) -> TaiResult<Output> {
    Command::new(LLDB)
        .arg("-s")
        .arg(source.as_ref())
        .output()
        .map_err(Into::into)
}
