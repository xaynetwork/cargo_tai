use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use crate::{common::tools::command_ext::ExitStatusExt, TaiResult};

const XCODEGEN: &str = "xcodegen";

#[derive(Default)]
pub struct XCodeGenGenerate {
    spec: Option<PathBuf>,
    project: Option<PathBuf>,
    verbose: bool,
}

impl XCodeGenGenerate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spec<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.spec = Some(path.as_ref().to_owned());
        self
    }

    pub fn project<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.project = Some(path.as_ref().to_owned());
        self
    }

    pub fn verbose(&mut self) -> &mut Self {
        self.verbose = true;
        self
    }

    pub fn execute(&mut self) -> TaiResult<()> {
        let mut cmd = Command::new(XCODEGEN);
        if !self.verbose {
            cmd.stdout(Stdio::null());
            cmd.stderr(Stdio::null());
        }

        cmd.arg("generate");
        self.spec.as_ref().map(|path| cmd.arg("--spec").arg(path));
        self.project
            .as_ref()
            .map(|path| cmd.arg("--project").arg(path));

        cmd.status()?.expect_success("failed to run xcodegen")
    }
}
