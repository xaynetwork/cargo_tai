use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use crate::{common::tools::command_ext::ExitStatusExt, TaiResult};

const ZIP: &str = "zip";

#[derive(Default)]
pub struct Zip {
    zip_file: Option<PathBuf>,
    file: Option<PathBuf>,
    current_dir: Option<PathBuf>,
    move_into_zip_file: bool,
    recurse_paths: bool,
    verbose: bool,
}

impl Zip {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn zip_file<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.zip_file = Some(path.as_ref().to_owned());
        self
    }

    pub fn file<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.file = Some(path.as_ref().to_owned());
        self
    }

    pub fn current_dir<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.current_dir = Some(path.as_ref().to_owned());
        self
    }

    pub fn recurse_paths(&mut self) -> &mut Self {
        self.recurse_paths = true;
        self
    }

    pub fn move_into_zip_file(&mut self) -> &mut Self {
        self.move_into_zip_file = true;
        self
    }

    pub fn verbose(&mut self) -> &mut Self {
        self.verbose = true;
        self
    }

    pub fn execute(self) -> TaiResult<()> {
        let mut command = Command::new(ZIP);
        if !self.verbose {
            command.stdout(Stdio::null());
            command.stderr(Stdio::null());
        }

        self.current_dir.map(|path| command.current_dir(path));
        self.recurse_paths.then(|| ()).map(|_| command.arg("-r"));
        self.move_into_zip_file
            .then(|| ())
            .map(|_| command.arg("-m"));
        self.zip_file.map(|path| command.arg(path));
        self.file.map(|path| command.arg(path));

        command.status()?.expect_success("failed to run zip")
    }
}
