use std::{path::Path, process::Command};

use crate::{common::tools::command_ext::ExitStatusExt, TaiResult};

const ZIP: &str = "zip";

pub fn zip<P1: AsRef<Path>, P2: AsRef<Path>>(dest: P1, file: P2) -> TaiResult<()> {
    Command::new(ZIP)
        .arg("-r")
        .arg(dest.as_ref())
        .arg(file.as_ref())
        .status()?
        .expect_success("failed to zip")
}
