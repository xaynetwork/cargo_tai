use std::{
    path::{Path, PathBuf},
    process::Command,
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

    pub fn archive(mut self) -> Self {
        self.archive = true;
        self
    }

    pub fn delete(mut self) -> Self {
        self.delete = true;
        self
    }

    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    pub fn only_content(mut self) -> Self {
        self.only_content = true;
        self
    }

    pub fn execute(self) -> TaiResult<()> {
        let mut command = Command::new(RSYNC);

        self.archive.then(|| ()).map(|_| command.arg("-a"));
        self.verbose.then(|| ()).map(|_| command.arg("-v"));
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
