use anyhow::bail;
use cfg_expr::targets::{Arch, Os};
use tracing::debug;

use crate::{android, common::opts::Options, ios, TaiResult};

#[derive(Debug, Clone)]
pub enum Command {
    Bench,
    Test,
    Benches,
    Tests,
}

pub fn run_command(requested: Options) -> TaiResult<()> {
    debug!("run command with options:\n{:?}", requested);
    match (requested.compiler.target.arch, requested.compiler.target.os) {
        #[cfg(feature = "ios")]
        (Arch::aarch64, Some(Os::ios)) => ios::platform::physical::run_command(requested),
        #[cfg(feature = "ios")]
        (Arch::x86_64, Some(Os::ios)) => ios::platform::simulator::run_command(requested),
        (Arch::aarch64 | Arch::arm | Arch::x86 | Arch::x86_64, Some(Os::android)) => {
            android::platform::run_command(requested)
        }
        _ => bail!("unsupported target: {:?}", requested.compiler.target),
    }
}
