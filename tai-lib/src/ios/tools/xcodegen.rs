use std::{
    path::{Path, PathBuf},
    process::Command,
};

use crate::{common::tools::command_ext::ExitStatusExt, TaiResult};

const XCODEGEN: &str = "xcodegen";

#[derive(Default)]
pub struct XCodeGenGenerate {
    spec: Option<PathBuf>,
    project: Option<PathBuf>,
}

impl XCodeGenGenerate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spec<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.spec = Some(path.as_ref().to_owned());
        self
    }

    pub fn project<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.project = Some(path.as_ref().to_owned());
        self
    }

    pub fn execute(self) -> TaiResult<()> {
        let mut command = Command::new(XCODEGEN);
        command.arg("generate");
        self.spec.map(|path| command.arg("--spec").arg(path));
        self.project.map(|path| command.arg("--project").arg(path));

        command.status()?.expect_success("failed to run xcodegen")
    }
}
