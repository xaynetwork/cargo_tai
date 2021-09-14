use std::{convert::TryFrom, path::Path, process::Command};

use anyhow::bail;
use cfg_expr::targets::{Arch, TargetInfo};

use crate::{
    common::{project::Profile, tools::command_ext::ExitStatusExt},
    TaiResult,
};

const XCODEBUILD: &str = "xcodebuild";

pub fn build<P: AsRef<Path>, D: AsRef<Path>>(
    project: P,
    scheme: &str,
    profile: &Profile,
    sdk: &Sdk,
    data_path: D,
) -> TaiResult<()> {
    Command::new(XCODEBUILD)
        .arg("-project")
        .arg(project.as_ref())
        .arg("-scheme")
        .arg(scheme)
        .arg("-configuration")
        .arg(profile.as_str())
        .arg("-sdk")
        .arg(sdk.as_str())
        .arg("-derivedDataPath")
        .arg(data_path.as_ref())
        .status()?
        .expect_success("failed to xcode")
}

pub fn build_for_testing<P: AsRef<Path>, D: AsRef<Path>>(
    project: P,
    scheme: &str,
    sdk: &Sdk,
    data_path: D,
) -> TaiResult<()> {
    Command::new(XCODEBUILD)
        .arg("-project")
        .arg(project.as_ref())
        .arg("-scheme")
        .arg(scheme)
        .arg("-sdk")
        .arg(sdk.as_str())
        .arg("-derivedDataPath")
        .arg(data_path.as_ref())
        .arg("build-for-testing")
        // .arg("-allowProvisioningUpdate")
        .status()?
        .expect_success("failed to xcode")
}

pub enum Sdk {
    Iphoneos,
    Iphonesimulator,
}

impl Sdk {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Iphoneos => "iphoneos",
            Self::Iphonesimulator => "iphonesimulator",
        }
    }
}

impl TryFrom<&TargetInfo<'_>> for Sdk {
    type Error = anyhow::Error;

    fn try_from(value: &TargetInfo<'_>) -> Result<Self, Self::Error> {
        match value.arch {
            Arch::aarch64 => Ok(Sdk::Iphoneos),
            Arch::x86_64 => Ok(Sdk::Iphonesimulator),
            _ => bail!("unsupported target"),
        }
    }
}
