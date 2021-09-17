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

    pub fn execute(self) -> TaiResult<()> {
        let mut command = Command::new(XCODEGEN);
        if !self.verbose {
            command.stdout(Stdio::null());
            command.stderr(Stdio::null());
        }

        command.arg("generate");
        self.spec.map(|path| command.arg("--spec").arg(path));
        self.project.map(|path| command.arg("--project").arg(path));

        command.status()?.expect_success("failed to run xcodegen")
    }
}
