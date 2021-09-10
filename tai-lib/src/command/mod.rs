use std::convert::TryInto;

use anyhow::bail;
use cfg_expr::targets::{Arch, Os};
use tracing::debug;

use crate::{android, ios, options::Options, TaiResult};

#[derive(Debug)]
pub enum Command {
    Bench,
    Test,
    Benches,
    Tests,
}

pub fn run_command(mut requested: Options) -> TaiResult<()> {
    debug!("run command with options:\n{:?}", requested);

    if let Command::Bench | Command::Benches = requested.general.command {
        let mut args_with_bench = vec!["--bench".to_string()];
        if let Some(ref args) = requested.general.binary.args {
            args_with_bench.extend_from_slice(args);
        };

        requested.general.binary.args = Some(args_with_bench);
    }

    match (
        requested.general.compiler.target.arch,
        requested.general.compiler.target.os,
    ) {
        #[cfg(feature = "ios")]
        (Arch::aarch64, Some(Os::ios)) => ios::platform::physical::run_command(requested),
        #[cfg(feature = "ios")]
        (Arch::x86_64, Some(Os::ios)) => ios::platform::simulator::run_command(requested),
        (Arch::aarch64 | Arch::arm | Arch::x86 | Arch::x86_64, Some(Os::android)) => {
            android::platform::run_command(requested.try_into()?)
        }
        _ => bail!(
            "unsupported target: {:?}",
            requested.general.compiler.target
        ),
    }
}
