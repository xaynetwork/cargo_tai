use std::{
    convert::TryFrom,
    path::{Path, PathBuf},
    process::{Command, Stdio},
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
    allow_provisioning_updates: bool,
    verbose: bool,
}

impl XCodeBuild {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn project<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.project = Some(path.as_ref().to_owned());
        self
    }

    pub fn scheme(&mut self, scheme: &str) -> &mut Self {
        self.scheme = Some(scheme.to_owned());
        self
    }

    pub fn profile(&mut self, profile: Profile) -> &mut Self {
        self.profile = Some(profile);
        self
    }

    pub fn sdk(&mut self, sdk: Sdk) -> &mut Self {
        self.sdk = Some(sdk);
        self
    }

    pub fn derived_data_path<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.derived_data_path = Some(path.as_ref().to_owned());
        self
    }

    pub fn build_for_testing(&mut self) -> &mut Self {
        self.build_for_testing = true;
        self
    }

    pub fn allow_provisioning_updates(&mut self) -> &mut Self {
        self.allow_provisioning_updates = true;
        self
    }

    pub fn verbose(&mut self) -> &mut Self {
        self.verbose = true;
        self
    }

    pub fn execute(self) -> TaiResult<()> {
        let mut command = Command::new(XCODEBUILD);
        if !self.verbose {
            command.stdout(Stdio::null());
            command.stderr(Stdio::null());
        }

        self.project.map(|path| command.arg("-project").arg(path));
        self.scheme.map(|scheme| command.arg("-scheme").arg(scheme));
        self.profile
            .map(|profile| command.arg("-configuration").arg(profile.as_str()));
        self.sdk.map(|sdk| command.arg("-sdk").arg(sdk.as_str()));
        self.derived_data_path
            .map(|path| command.arg("-derivedDataPath").arg(path));
        self.allow_provisioning_updates
            .then(|| ())
            .map(|_| command.arg("-allowProvisioningUpdates"));
        self.build_for_testing
            .then(|| ())
            .map(|_| command.arg("build-for-testing"));

        command.status()?.expect_success("failed to run xcode")
    }
}
