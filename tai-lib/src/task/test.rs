use anyhow::bail;
use cfg_expr::targets::{Arch, Vendor};

use crate::{ios::platform, TaiResult};

use super::Options;

pub fn run_test(requested: &Options) -> TaiResult<()> {
    match (requested.target.arch, requested.target.vendor) {
        (Arch::aarch64, Some(Vendor::apple)) => platform::physical::run_test(requested),
        (Arch::x86_64, Some(Vendor::apple)) => platform::simulator::run_test(requested),
        _ => bail!("unsupported target: {:?}", requested.target),
    }
}
