use std::{
    path::Path,
    process::{Command, Output},
};

use crate::TaiResult;

const LLDB: &str = "lldb";

pub fn run_source<P: AsRef<Path>>(source: P) -> TaiResult<Output> {
    Command::new(LLDB)
        .arg("-s")
        .arg(source.as_ref())
        .output()
        .map_err(Into::into)
}
