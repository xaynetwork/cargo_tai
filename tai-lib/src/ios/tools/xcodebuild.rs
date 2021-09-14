use std::{path::Path, process::Command};

use crate::{
    common::{project::Profile, tools::command_ext::ExitStatusExt},
    TaiResult,
};

const XCODEBUILD: &str = "xcodebuild";

pub fn build<P: AsRef<Path>, D: AsRef<Path>>(
    project: P,
    scheme: &str,
    profile: &Profile,
    data_path: D,
) -> TaiResult<()> {
    Command::new(XCODEBUILD)
        .arg("-project")
        .arg(project.as_ref())
        .arg("-scheme")
        .arg(scheme)
        .arg("-configuration")
        .arg(profile.as_str())
        .arg("-derivedDataPath")
        .arg(data_path.as_ref())
        .status()?
        .expect_success("failed to xcode")
}

pub fn build_for_testing<P: AsRef<Path>, D: AsRef<Path>>(
    project: P,
    scheme: &str,
    data_path: D,
) -> TaiResult<()> {
    Command::new(XCODEBUILD)
        .arg("-project")
        .arg(project.as_ref())
        .arg("-scheme")
        .arg(scheme)
        .arg("-derivedDataPath")
        .arg(data_path.as_ref())
        .arg("build-for-testing")
        // .arg("-allowProvisioningUpdate")
        .status()?
        .expect_success("failed to xcode")
}
