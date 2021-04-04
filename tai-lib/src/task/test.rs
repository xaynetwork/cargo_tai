use anyhow::bail;
use cfg_expr::targets::{Arch, Os};

use crate::{android, ios, TaiResult};

use super::Options;

pub fn run_test(requested: &Options) -> TaiResult<()> {
    match (requested.target.arch, requested.target.os) {
        (Arch::aarch64, Some(Os::ios)) => ios::platform::physical::run_test(requested),
        (Arch::x86_64, Some(Os::ios)) => ios::platform::simulator::run_test(requested),
        (Arch::aarch64, Some(Os::android)) => android::platform::run_test(requested),
        (Arch::arm, Some(Os::android)) => android::platform::run_test(requested),
        (Arch::x86, Some(Os::android)) => android::platform::run_test(requested),
        (Arch::x86_64, Some(Os::android)) => android::platform::run_test(requested),
        _ => bail!("unsupported target: {:?}", requested.target),
    }
}
