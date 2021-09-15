use std::{
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::bail;
use tracing::{debug, instrument};

use crate::{
    android::tools::{adb, AndroidSdk},
    common::{bundle::BuildBundle, options::BinaryOptions, task::Task},
    TaiResult,
};

use super::Context;

const ANDROID_REMOTE_WORKDIR: &str = "/data/local/tmp/cargo-tai";

pub struct RunOnDevices;

impl Task<Context> for RunOnDevices {
    fn run(&self, context: Context) -> TaiResult<Context> {
        let sdk = context.android_sdk()?;
        let bundles = context.build_bundles()?;

        context.devices()?.iter().try_for_each(|device| {
            bundles.bundles.iter().try_for_each(|bundle| {
                install_and_run_bundle(sdk, &device.id, bundle, context.binary()?)
            })
        })?;
        Ok(context)
    }
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
