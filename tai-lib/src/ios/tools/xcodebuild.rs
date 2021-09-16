use std::{
    convert::TryFrom,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::bail;
use cfg_expr::targets::TargetInfo;

use crate::{
    common::{project::Profile, tools::command_ext::ExitStatusExt},
    TaiResult,
};

const XCODEBUILD: &str = "xcodebuild";

#[derive(Debug, Clone, Copy)]
pub enum Sdk {
    IPhoneOS,
    IPhoneSimulator,
}

impl Sdk {
    pub fn as_str(&self) -> &str {
        match self {
            Self::IPhoneOS => "iphoneos",
            Self::IPhoneSimulator => "iphonesimulator",
        }
    }
}

impl TryFrom<&TargetInfo<'_>> for Sdk {
    type Error = anyhow::Error;

    fn try_from(value: &TargetInfo<'_>) -> Result<Self, Self::Error> {
        match value.triple {
            "aarch64-apple-ios" => Ok(Sdk::IPhoneOS),
            "x86_64-apple-ios" => Ok(Sdk::IPhoneSimulator),
            _ => bail!("unsupported target"),
        }
    }
}

#[derive(Default)]
pub struct XCodeBuild {
    project: Option<PathBuf>,
    scheme: Option<String>,
    profile: Option<Profile>,
    sdk: Option<Sdk>,
    derived_data_path: Option<PathBuf>,
    build_for_testing: bool,
    allow_provisioning_update: bool,
}

impl XCodeBuild {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn project<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.project = Some(path.as_ref().to_owned());
        self
    }

    pub fn scheme(mut self, scheme: &str) -> Self {
        self.scheme = Some(scheme.to_owned());
        self
    }

    pub fn profile(mut self, profile: Profile) -> Self {
        self.profile = Some(profile);
        self
    }

    pub fn sdk(mut self, sdk: Sdk) -> Self {
        self.sdk = Some(sdk);
        self
    }

    pub fn derived_data_path<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.derived_data_path = Some(path.as_ref().to_owned());
        self
    }

    pub fn build_for_testing(mut self) -> Self {
        self.build_for_testing = true;
        self
    }

    pub fn allow_provisioning_update(mut self) -> Self {
        self.allow_provisioning_update = true;
        self
    }

    pub fn execute(self) -> TaiResult<()> {
        let mut command = Command::new(XCODEBUILD);
        self.project.map(|path| command.arg("-project").arg(path));
        self.scheme.map(|scheme| command.arg("-scheme").arg(scheme));
        self.profile
            .map(|profile| command.arg("-configuration").arg(profile.as_str()));
        self.sdk.map(|sdk| command.arg("-sdk").arg(sdk.as_str()));
        self.derived_data_path
            .map(|path| command.arg("-derivedDataPath").arg(path));
        self.allow_provisioning_update
            .then(|| ())
            .map(|_| command.arg("-allowProvisioningUpdate"));
        self.build_for_testing
            .then(|| ())
            .map(|_| command.arg("build-for-testing"));

        command.status()?.expect_success("failed to run xcode")
    }
}
