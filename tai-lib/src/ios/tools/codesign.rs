use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use crate::{common::tools::command_ext::ExitStatusExt, TaiResult};

const CODE_SIGN: &str = "codesign";

#[derive(Default)]
pub struct CodeSign<'f> {
    identity: String,
    files: &'f [&'f PathBuf],
    entitlements: Option<PathBuf>,
    verbose: bool,
}

impl<'f> CodeSign<'f> {
    pub fn new(identity: &str, files: &'f [&'f PathBuf]) -> Self {
        Self {
            identity: identity.to_owned(),
            files,
            ..Default::default()
        }
    }

    pub fn entitlements<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.entitlements = Some(path.as_ref().to_owned());
        self
    }

    // pub fn verbose(&mut self) -> &mut Self {
    //     self.verbose = true;
    //     self
    // }

    pub fn execute(&mut self) -> TaiResult<()> {
        let mut cmd = Command::new(CODE_SIGN);
        if !self.verbose {
            cmd.stdout(Stdio::null());
            cmd.stderr(Stdio::null());
        }

        cmd.args(["-s", self.identity.as_ref()]);
        self.entitlements
            .as_ref()
            .map(|path| cmd.arg("--entitlements").arg(path));
        cmd.args(self.files);

        cmd.status()?.expect_success("failed to run codesign")
    }
}
