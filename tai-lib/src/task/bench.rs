use anyhow::bail;
use cfg_expr::targets::{Arch, Os};

use crate::TaiResult;

#[cfg(feature = "ios")]
use crate::ios;

use super::Options;

pub fn run_benches(requested: &Options) -> TaiResult<()> {
    match (requested.target.arch, requested.target.os) {
        #[cfg(feature = "ios")]
        (Arch::aarch64, Some(Os::ios)) => ios::platform::physical::run_benches(requested),
        #[cfg(feature = "ios")]
        (Arch::x86_64, Some(Os::ios)) => ios::platform::simulator::run_benches(requested),
        _ => bail!("unsupported target: {:?}", requested.target),
    }
}
