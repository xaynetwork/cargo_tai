use anyhow::{anyhow, bail};
use std::{io::Write, path::PathBuf};
use tracing::{debug, instrument};

const ANDROID_REMOTE_WORKDIR: &str = "/data/local/tmp/cargo-tai";

use crate::{
    android::{
        bundle::create_bundle,
        tools::{adb, AndroidSdk},
    },
    bundle::{create_bundles, BuildBundle},
    compiler::{compile_benches, compile_tests, BuildUnit},
    task::Options,
    TaiResult,
};

use super::compiler::{bench_command, test_command};

#[instrument(name = "benches", skip(requested))]
pub fn run_benches(requested: &Options) -> TaiResult<()> {
    let sdk = AndroidSdk::derive_sdk(&requested.android.ndk)?;
    let build_units = compile_benches(bench_command(&sdk, requested)?, requested)?;

    let mut args_with_bench = vec!["--bench".to_string()];
    if let Some(ref args) = requested.args {
        args_with_bench.extend_from_slice(args);
    };

    run(
        &sdk,
        build_units,
        &Some(args_with_bench),
        &requested.envs,
        &requested.resources,
    )
}

#[instrument(name = "tests", skip(requested))]
pub fn run_test(requested: &Options) -> TaiResult<()> {
    let sdk = AndroidSdk::derive_sdk(&requested.android.ndk)?;
    let build_units = compile_tests(test_command(&sdk, requested)?, requested)?;

    run(
        &sdk,
        build_units,
        &requested.args,
        &requested.envs,
        &requested.resources,
    )
}

#[instrument(name = "run", skip(sdk, build_units))]
pub fn run(
    sdk: &AndroidSdk,
    build_units: Vec<BuildUnit>,
    args: &Option<Vec<String>>,
    envs: &Option<Vec<(String, String)>>,
    resources: &Option<Vec<(String, PathBuf)>>,
) -> TaiResult<()> {
    let devices = adb::devices(&sdk)?
        .pop()
        .ok_or_else(|| anyhow!("no android device available"))?;

    let bundles = create_bundles(build_units, |unit, root| {
        create_bundle(unit, root, resources)
    })?;

    bundles
        .bundles
        .iter()
        .try_for_each(|bundle| install_and_launch(&sdk, &devices.id, &bundle, args, envs))
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
    adb::mkdir(&sdk, device, &remote_workdir)?;

    let remote_root = remote_workdir.join(&bundle.root.file_name().unwrap());
    debug!("copy from: {:?} to: {:?}", bundle.root, remote_root);
    adb::sync(&sdk, device, &bundle.root, &remote_root)?;
    let remote_exe = remote_root.join(&bundle.build_unit.name);
    debug!("chmod {:?}", remote_exe);
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

    let result = adb::run(&sdk, device, &start_script)?;
    let _ = std::io::stdout().write(result.stdout.as_slice());
    let _ = std::io::stderr().write(result.stderr.as_slice());

    adb::rm(&sdk, device, &remote_root)?;

    if result.status.success() {
        Ok(())
    } else {
        bail!("test failed")
    }
}
