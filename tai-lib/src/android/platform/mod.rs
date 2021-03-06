use anyhow::{anyhow, bail};
use std::{
    convert::TryFrom,
    io::Write,
    path::{Path, PathBuf},
};
use tracing::{debug, instrument};

const ANDROID_REMOTE_WORKDIR: &str = "/data/local/tmp/cargo-tai";

use crate::{
    android::{
        bundle::create_bundle,
        tools::{
            adb::{self, Device},
            AndroidSdk,
        },
    },
    bundle::{create_bundles, BuildBundle},
    compiler::{compile_benches, compile_tests},
    task::{self, BinaryOptions, GeneralOptions, Task},
    TaiResult,
};

use super::compiler::{bench_command, benches_command, test_command, tests_command};

#[instrument(name = "build_and_run", skip(requested))]
pub fn run_task(requested: Options) -> TaiResult<()> {
    let sdk = AndroidSdk::derive_sdk(&requested.android_ndk)?;

    let cmd = match requested.general.task {
        Task::Bench => bench_command(&sdk, &requested)?,
        Task::Test => test_command(&sdk, &requested)?,
        Task::Benches => benches_command(&sdk, &requested)?,
        Task::Tests => tests_command(&sdk, &requested)?,
    };

    let build_units = match requested.general.task {
        Task::Bench | Task::Benches => compile_benches(cmd, &requested.general.compiler)?,
        Task::Test | Task::Tests => compile_tests(cmd, &requested.general.compiler)?,
    };

    let bundles = create_bundles(build_units, |unit, root| {
        create_bundle(unit, root, &requested.general.binary.resources)
    })?;

    let devices = adb::devices(&sdk)?
        .into_iter()
        .filter(|device| device.arch == requested.general.compiler.target.arch)
        .collect::<Vec<Device>>();

    if devices.is_empty() {
        bail!("no android device available")
    }

    devices.iter().try_for_each(|device| {
        bundles.bundles.iter().try_for_each(|bundle| {
            install_and_run_bundle(&sdk, &device.id, bundle, &requested.general.binary)
        })
    })
}

fn install_and_run_bundle(
    sdk: &AndroidSdk,
    device: &str,
    bundle: &BuildBundle,
    binary_opt: &BinaryOptions,
) -> TaiResult<()> {
    let (remote_root, remote_exe) = install_bundle(sdk, device, bundle)?;
    let result = run_bundle(sdk, device, binary_opt, &remote_root, &remote_exe)?;

    adb::rm(sdk, device, &remote_root)?;

    if result.status.success() {
        Ok(())
    } else {
        bail!("test failed")
    }
}

#[instrument(name = "install", skip(sdk, bundle))]
fn install_bundle(
    sdk: &AndroidSdk,
    device: &str,
    bundle: &BuildBundle,
) -> TaiResult<(PathBuf, PathBuf)> {
    let remote_workdir = PathBuf::from(ANDROID_REMOTE_WORKDIR);
    adb::mkdir(sdk, device, &remote_workdir)?;
    let remote_root = remote_workdir.join(&bundle.root.file_name().unwrap());
    debug!(
        "copy from: {} to: {}",
        bundle.root.display(),
        remote_root.display()
    );
    adb::sync(sdk, device, &bundle.root, &remote_root)?;
    let remote_exe = remote_root.join(&bundle.build_unit.name);
    debug!("chmod {}", remote_exe.display());
    adb::chmod(sdk, device, &remote_exe)?;
    Ok((remote_root, remote_exe))
}

fn run_bundle(
    sdk: &AndroidSdk,
    device: &str,
    binary_opt: &BinaryOptions,
    remote_root: &Path,
    remote_exe: &Path,
) -> TaiResult<std::process::Output> {
    let envs_as_string = if let Some(envs) = &binary_opt.envs {
        envs.iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<String>>()
            .join(" ")
    } else {
        String::from("")
    };

    let start_script = format!(
        include_str!("../templates/start_script.tmpl"),
        remote_bundle_root = remote_root.to_string_lossy(),
        envs = envs_as_string,
        remote_executable = remote_exe.to_string_lossy(),
        args = binary_opt.args.as_ref().unwrap_or(&vec![]).join(" ")
    );
    let result = adb::run(sdk, device, &start_script)?;
    let _ = std::io::stdout().write(result.stdout.as_slice());
    let _ = std::io::stderr().write(result.stderr.as_slice());
    Ok(result)
}

pub struct Options {
    pub general: GeneralOptions,

    pub android_api_lvl: u8,
    pub android_ndk: PathBuf,
}

impl TryFrom<task::Options> for Options {
    type Error = anyhow::Error;

    fn try_from(opt: task::Options) -> Result<Self, Self::Error> {
        Ok(Self {
            general: opt.general,
            android_api_lvl: opt
                .platform
                .android_api_lvl
                .ok_or_else(|| anyhow!("the option android_api_lvl is missing"))?,
            android_ndk: opt
                .platform
                .android_ndk
                .ok_or_else(|| anyhow!("the option android_ndk is missing"))?,
        })
    }
}
