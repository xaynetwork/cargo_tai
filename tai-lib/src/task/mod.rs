use std::{convert::TryInto, path::PathBuf};

use anyhow::bail;
use cfg_expr::targets::{Arch, Os, TargetInfo};
use tracing::debug;

use crate::{android, ios, TaiResult};

#[derive(Debug)]
pub enum Task {
    Bench,
    Test,
    Benches,
    Tests,
}

#[derive(Debug)]
pub struct Options {
    pub general: GeneralOptions,

    pub platform: PlatformOptions,
}

#[derive(Debug)]
pub struct GeneralOptions {
    pub task: Task,
    pub compiler: CompilerOptions,
    pub binary: BinaryOptions,
}

#[derive(Debug)]
pub struct CompilerOptions {
    /// Build for the target triples
    pub target: TargetInfo<'static>,

    // cargo arguments
    pub cargo_args: Vec<String>,
}

#[derive(Debug)]
pub struct BinaryOptions {
    pub args: Option<Vec<String>>,
    pub envs: Option<Vec<(String, String)>>,
    pub resources: Option<Vec<(String, PathBuf)>>,
}

#[derive(Debug)]
pub struct PlatformOptions {
    pub android_api_lvl: Option<u8>,
    pub android_ndk: Option<PathBuf>,
    pub ios_mobile_provision: Option<PathBuf>,
}

pub fn run_task(mut requested: Options) -> TaiResult<()> {
    debug!("run with options:\n{:?}", requested);

    if let Task::Bench | Task::Benches = requested.general.task {
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
        (Arch::aarch64, Some(Os::ios)) => ios::platform::physical::run_task(requested.try_into()?),
        #[cfg(feature = "ios")]
        (Arch::x86_64, Some(Os::ios)) => ios::platform::simulator::run_task(requested.try_into()?),
        (Arch::aarch64 | Arch::arm | Arch::x86 | Arch::x86_64, Some(Os::android)) => {
            android::platform::run_task(requested.try_into()?)
        }
        _ => bail!(
            "unsupported target: {:?}",
            requested.general.compiler.target
        ),
    }
}
