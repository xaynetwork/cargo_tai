use std::process::ExitStatus;

use anyhow::{bail, Result};

pub trait ExitStatusExt {
    fn expect_success(&self, message: &str) -> Result<()>;
}

impl ExitStatusExt for ExitStatus {
    fn expect_success(&self, message: &str) -> Result<()> {
        match self.success() {
            true => Ok(()),
            false => bail!("{}", message),
        }
    }
}
