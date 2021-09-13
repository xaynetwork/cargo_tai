use std::{path::Path, process::Command};

use crate::{command_ext::ExitStatusExt, TaiResult};

const RSYNC: &str = "rsync";

pub fn rsync<P1: AsRef<Path>, P2: AsRef<Path>>(source: P1, dest: P2) -> TaiResult<()> {
    Command::new(RSYNC)
        .args(&["-av", "--delete"])
        .arg(format!("{}/", source.as_ref().display()))
        .args(&[dest.as_ref()])
        .status()?
        .expect_success("failed to rsync")
}
