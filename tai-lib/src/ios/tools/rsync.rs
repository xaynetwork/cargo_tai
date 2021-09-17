use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use crate::{common::tools::command_ext::ExitStatusExt, TaiResult};

const RSYNC: &str = "rsync";

pub struct Rsync {
    source: PathBuf,
    destination: PathBuf,
    archive: bool,
    delete: bool,
    verbose: bool,
    only_content: bool,
}

impl Rsync {
    pub fn new<S, D>(source: S, destination: D) -> Self
    where
        S: AsRef<Path>,
        D: AsRef<Path>,
    {
        Self {
            source: source.as_ref().to_owned(),
            destination: destination.as_ref().to_owned(),
            archive: false,
            delete: false,
            verbose: false,
            only_content: false,
        }
    }

    pub fn archive(&mut self) -> &mut Self {
        self.archive = true;
        self
    }

    pub fn delete(&mut self) -> &mut Self {
        self.delete = true;
        self
    }

    pub fn verbose(&mut self) -> &mut Self {
        self.verbose = true;
        self
    }

    pub fn only_content(&mut self) -> &mut Self {
        self.only_content = true;
        self
    }

    pub fn execute(self) -> TaiResult<()> {
        let mut command = Command::new(RSYNC);
        if !self.verbose {
            command.stdout(Stdio::null());
            command.stderr(Stdio::null());
        } else {
            command.arg("-v");
        }

        self.archive.then(|| ()).map(|_| command.arg("-a"));
        self.delete.then(|| ()).map(|_| command.arg("--delete"));

        if self.only_content {
            command.arg(format!("{}/", self.source.display()));
        } else {
            command.arg(self.source);
        };

        command.arg(self.destination);
        command.status()?.expect_success("failed to run rsync")
    }
}
