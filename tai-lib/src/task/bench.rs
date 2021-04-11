use anyhow::bail;
use cfg_expr::targets::{Arch, Os};

use crate::TaiResult;

#[cfg(target_os = "macos")]
use crate::ios;

use super::Options;

pub fn run_benches(requested: &Options) -> TaiResult<()> {
    match (requested.target.arch, requested.target.os) {
        #[cfg(target_os = "macos")]
        (Arch::aarch64, Some(Os::ios)) => ios::platform::physical::run_bench(requested),
        #[cfg(target_os = "macos")]
        (Arch::x86_64, Some(Os::ios)) => ios::platform::simulator::run_bench(requested),
        _ => bail!("unsupported target: {:?}", requested.target),
    }
}
