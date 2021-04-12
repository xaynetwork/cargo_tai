use anyhow::bail;
use cfg_expr::targets::{Arch, Os};

use crate::{android, TaiResult};

#[cfg(feature = "ios")]
use crate::ios;

use super::Options;

pub fn run_tests(requested: &Options) -> TaiResult<()> {
    match (requested.target.arch, requested.target.os) {
        #[cfg(feature = "ios")]
        (Arch::aarch64, Some(Os::ios)) => ios::platform::physical::run_tests(requested),
        #[cfg(feature = "ios")]
        (Arch::x86_64, Some(Os::ios)) => ios::platform::simulator::run_tests(requested),
        (Arch::aarch64, Some(Os::android)) => android::platform::run_test(requested),
        (Arch::arm, Some(Os::android)) => android::platform::run_test(requested),
        (Arch::x86, Some(Os::android)) => android::platform::run_test(requested),
        (Arch::x86_64, Some(Os::android)) => android::platform::run_test(requested),
        _ => bail!("unsupported target: {:?}", requested.target),
    }
}
