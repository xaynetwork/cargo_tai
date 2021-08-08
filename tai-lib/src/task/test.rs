use std::convert::TryInto;

use anyhow::bail;
use cfg_expr::targets::{Arch, Os};

use crate::{android, TaiResult};

#[cfg(feature = "ios")]
use crate::ios;

use super::Options;

pub fn run_test(requested: Options) -> TaiResult<()> {
    match (
        requested.general.compiler.target.arch,
        requested.general.compiler.target.os,
    ) {
        #[cfg(feature = "ios")]
        (Arch::aarch64, Some(Os::ios)) => ios::platform::physical::run_test(requested.try_into()?),
        #[cfg(feature = "ios")]
        (Arch::x86_64, Some(Os::ios)) => ios::platform::simulator::run_test(requested.try_into()?),
        (Arch::aarch64, Some(Os::android)) => android::platform::run_test(requested.try_into()?),
        (Arch::arm, Some(Os::android)) => android::platform::run_test(requested.try_into()?),
        (Arch::x86, Some(Os::android)) => android::platform::run_test(requested.try_into()?),
        (Arch::x86_64, Some(Os::android)) => android::platform::run_test(requested.try_into()?),
        _ => bail!(
            "unsupported target: {:?}",
            requested.general.compiler.target
        ),
    }
}

pub fn run_tests(requested: Options) -> TaiResult<()> {
    match (
        requested.general.compiler.target.arch,
        requested.general.compiler.target.os,
    ) {
        #[cfg(feature = "ios")]
        (Arch::aarch64, Some(Os::ios)) => ios::platform::physical::run_tests(requested.try_into()?),
        #[cfg(feature = "ios")]
        (Arch::x86_64, Some(Os::ios)) => ios::platform::simulator::run_tests(requested.try_into()?),
        (Arch::aarch64, Some(Os::android)) => android::platform::run_tests(requested.try_into()?),
        (Arch::arm, Some(Os::android)) => android::platform::run_tests(requested.try_into()?),
        (Arch::x86, Some(Os::android)) => android::platform::run_tests(requested.try_into()?),
        (Arch::x86_64, Some(Os::android)) => android::platform::run_tests(requested.try_into()?),
        _ => bail!(
            "unsupported target: {:?}",
            requested.general.compiler.target
        ),
    }
}
