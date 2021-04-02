use std::process::{ExitStatus, Output};

use anyhow::{bail, Result};

pub trait OutputExt {
    fn expect_success(self) -> Result<Output>;
    // fn validate_with_output(self) -> Result<Output>;
}

impl OutputExt for Output {
    fn expect_success(self) -> Result<Output> {
        match self.status.success() {
            true => Ok(self),
            false => bail!("command failed"),
        }
    }

    // fn validate_with_output(self) -> Result<Output> {
    //     match self.status.success() {
    //         true => Ok(self),
    //         false => Err(Error::Output {
    //             stdout: String::from_utf8(self.stdout).unwrap(),
    //             stderr: String::from_utf8(self.stderr).unwrap(),
    //             status: self.status,
    //         }),
    //     }
    // }
}

pub trait ExitStatusExt {
    fn expect_success(&self) -> Result<()>;
}

impl ExitStatusExt for ExitStatus {
    fn expect_success(&self) -> Result<()> {
        match self.success() {
            true => Ok(()),
            false => bail!("command failed"),
        }
    }
}
