use std::{path::Path, process::Command};

use crate::{common::tools::command_ext::ExitStatusExt, TaiResult};

const GRADLEW: &str = "./gradlew";

pub fn assemble_debug<P: AsRef<Path>>(working_dir: P) -> TaiResult<()> {
    Command::new(GRADLEW)
        .arg("app:assembleDebug")
        .current_dir(working_dir.as_ref())
        .status()?
        .expect_success("failed to create directory")
}

pub fn assemble_android_test<P: AsRef<Path>>(working_dir: P) -> TaiResult<()> {
    Command::new(GRADLEW)
        .arg("app:assembleAndroidTest")
        .current_dir(working_dir.as_ref())
        .status()?
        .expect_success("failed to create directory")
}
