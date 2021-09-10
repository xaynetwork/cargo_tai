use std::{
    path::Path,
    process::{Command, Output},
};

use anyhow::anyhow;

use crate::TaiResult;

const XCODEBUILD: &str = "xcodebuild";

pub fn build<P: AsRef<Path>, D: AsRef<Path>>(
    project: P,
    scheme: &str,
    data_path: D,
) -> TaiResult<Output> {
    Command::new(XCODEBUILD)
        .arg("-workspace")
        .arg(project.as_ref())
        .arg("-scheme")
        .arg(scheme)
        .arg("-derivedDataPath")
        .arg(data_path.as_ref())
        .output()
        .map_err(|err| anyhow!("{}", err))
}

pub fn build_for_testing<P: AsRef<Path>, D: AsRef<Path>>(
    project: P,
    scheme: &str,
    data_path: D,
) -> TaiResult<Output> {
    Command::new(XCODEBUILD)
        .arg("-project")
        .arg(project.as_ref())
        .arg("-scheme")
        .arg(scheme)
        .arg("-derivedDataPath")
        .arg(data_path.as_ref())
        // .arg("-sdk")
        // .arg("iphoneos")
        .arg("build-for-testing")
        // .arg("-allowProvisioningUpdate")
        .output()
        .map_err(|err| anyhow!("{}", err))
}
