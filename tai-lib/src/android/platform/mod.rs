use anyhow::{anyhow, bail};
use std::{convert::TryFrom, io::Write, path::PathBuf, process::Command};
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
    compiler::{compile_benches, compile_tests, BuildUnit},
    task::{self, GeneralOptions},
    TaiResult,
};

use super::compiler::{bench_command, benches_command, test_command, tests_command};

#[instrument(name = "bench", skip(requested))]
pub fn run_bench(requested: Options) -> TaiResult<()> {
    let sdk = AndroidSdk::derive_sdk(&requested.android_ndk)?;
    compile_and_run_benches(&sdk, &requested, bench_command(&sdk, &requested)?)
}

#[instrument(name = "benches", skip(requested))]
pub fn run_benches(requested: Options) -> TaiResult<()> {
    let sdk = AndroidSdk::derive_sdk(&requested.android_ndk)?;
    compile_and_run_benches(&sdk, &requested, benches_command(&sdk, &requested)?)
}

#[instrument(name = "test", skip(requested))]
pub fn run_test(requested: Options) -> TaiResult<()> {
    let sdk = AndroidSdk::derive_sdk(&requested.android_ndk)?;
    compile_and_run_tests(&sdk, &requested, test_command(&sdk, &requested)?)
}

#[instrument(name = "tests", skip(requested))]
pub fn run_tests(requested: Options) -> TaiResult<()> {
    let sdk = AndroidSdk::derive_sdk(&requested.android_ndk)?;
    compile_and_run_tests(&sdk, &requested, tests_command(&sdk, &requested)?)
}

#[instrument(skip(sdk, requested, cmd))]
fn compile_and_run_benches(sdk: &AndroidSdk, requested: &Options, cmd: Command) -> TaiResult<()> {
    let build_units = compile_benches(cmd, &requested.general.compiler)?;

    let mut args_with_bench = vec!["--bench".to_string()];
    if let Some(ref args) = requested.general.binary.args {
        args_with_bench.extend_from_slice(args);
    };

    run(
        sdk,
        requested,
        build_units,
        &Some(args_with_bench),
        &requested.general.binary.envs,
        &requested.general.binary.resources,
    )
}

#[instrument(skip(sdk, requested, cmd))]
fn compile_and_run_tests(sdk: &AndroidSdk, requested: &Options, cmd: Command) -> TaiResult<()> {
    let build_units = compile_tests(cmd, &requested.general.compiler)?;

    run(
        sdk,
        requested,
        build_units,
        &requested.general.binary.args,
        &requested.general.binary.envs,
        &requested.general.binary.resources,
    )
}

#[instrument(name = "run", skip(sdk, build_units, requested))]
pub fn run(
    sdk: &AndroidSdk,
    requested: &Options,
    build_units: Vec<BuildUnit>,
    args: &Option<Vec<String>>,
    envs: &Option<Vec<(String, String)>>,
    resources: &Option<Vec<(String, PathBuf)>>,
) -> TaiResult<()> {
    let devices = adb::devices(sdk)?
        .into_iter()
        .filter(|device| device.arch == requested.general.compiler.target.arch)
        .collect::<Vec<Device>>()
        .pop()
        .ok_or_else(|| anyhow!("no android device available"))?;

    let bundles = create_bundles(build_units, |unit, root| {
        create_bundle(unit, root, resources)
    })?;

    bundles
        .bundles
        .iter()
        .try_for_each(|bundle| install_and_launch(sdk, &devices.id, bundle, args, envs))
}

#[instrument(name = "install_launch", skip(sdk, bundle))]
fn install_and_launch(
    sdk: &AndroidSdk,
    device: &str,
    bundle: &BuildBundle,
    args: &Option<Vec<String>>,
    envs: &Option<Vec<(String, String)>>,
) -> TaiResult<()> {
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

    let envs_as_string = if let Some(envs) = envs {
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
        args = args.as_ref().unwrap_or(&vec![]).join(" ")
    );

    let result = adb::run(sdk, device, &start_script)?;
    let _ = std::io::stdout().write(result.stdout.as_slice());
    let _ = std::io::stderr().write(result.stderr.as_slice());

    adb::rm(sdk, device, &remote_root)?;

    if result.status.success() {
        Ok(())
    } else {
        bail!("test failed")
    }
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
