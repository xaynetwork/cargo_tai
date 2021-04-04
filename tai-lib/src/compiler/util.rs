use std::process::Command;

use crate::{task::Options, TaiResult};

pub fn extend_with_options<'a>(
    cmd: &'a mut Command,
    requested: &Options,
) -> TaiResult<&'a mut Command> {
    if requested.release == true {
        cmd.arg("--release");
    }

    if requested.all_features == true {
        cmd.arg("--all-features");
    }

    if requested.no_default_features == true {
        cmd.arg("--no-default-features");
    }

    cmd.args(&[
        "--target",
        requested.target.triple,
        "--message-format",
        "json",
    ]);

    Ok(cmd)
}
