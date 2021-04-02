use std::{
    path::Path,
    process::{Command, Output},
};

use anyhow::anyhow;

use crate::TaiResult;

const LLDB: &'static str = "lldb";

pub fn run_source<P: AsRef<Path>>(source: P) -> TaiResult<Output> {
    Command::new(LLDB)
        .arg("-s")
        .arg(source.as_ref())
        .output()
        .map_err(|err| anyhow!("{}", err))
}
