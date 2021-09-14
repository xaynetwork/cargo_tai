use std::{path::Path, process::Command};

use crate::{common::tools::command_ext::ExitStatusExt, TaiResult};

const CODE_SIGN: &str = "codesign";

pub fn sign<P1: AsRef<Path>, P2: AsRef<Path>>(
    identity: &str,
    entitlements: P1,
    bundle: P2,
) -> TaiResult<()> {
    Command::new(CODE_SIGN)
        .args(&["-s", identity, "--entitlements"])
        .args(&[entitlements.as_ref(), bundle.as_ref()])
        .status()?
        .expect_success("failed to sign app")
}
