use std::{
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::bail;
use tracing::{debug, instrument};

use crate::{
    android::tools::{adb, AndroidEnv},
    common::{
        bundle::{BuiltBundle, BuiltBundles},
        opts::{BinaryOptions, Options},
        task::Task,
    },
    TaiResult,
};

use super::{list_devices::Devices, Context};

const ANDROID_REMOTE_WORKDIR: &str = "/data/local/tmp/cargo-tai";

pub struct RunOnDevices;

impl Task<Context> for RunOnDevices {
    fn run(&self, context: Context) -> TaiResult<Context> {
        let env: &AndroidEnv = context.get();
        let bundles = context.get::<BuiltBundles>();
        let default = BinaryOptions::default();
        let binary_opt = match context.get::<Options>().binary.as_ref() {
            Some(opts) => opts,
            None => &default,
        };

        context.get::<Devices>().0.iter().try_for_each(|device| {
            bundles
                .bundles
                .iter()
                .try_for_each(|bundle| install_and_run_bundle(env, &device.id, bundle, binary_opt))
        })?;
        Ok(context)
    }
}

fn install_and_run_bundle(
    env: &AndroidEnv,
    device: &str,
    bundle: &BuiltBundle,
    binary_opt: &BinaryOptions,
) -> TaiResult<()> {
    let (remote_root, remote_exe) = install_bundle(env, device, bundle)?;
    let result = run_bundle(env, device, binary_opt, &remote_root, &remote_exe)?;

    adb::rm(env, device, &remote_root)?;

    if result.status.success() {
        Ok(())
    } else {
        bail!("test failed")
    }
}

#[instrument(name = "install", skip(env, bundle))]
fn install_bundle(
    env: &AndroidEnv,
    device: &str,
    bundle: &BuiltBundle,
) -> TaiResult<(PathBuf, PathBuf)> {
    let remote_workdir = PathBuf::from(ANDROID_REMOTE_WORKDIR);
    adb::mkdir(env, device, &remote_workdir)?;
    let remote_root = remote_workdir.join(bundle.root.file_name().unwrap());
    debug!(
        "copy from: {} to: {}",
        bundle.root.display(),
        remote_root.display()
    );
    adb::sync(env, device, &bundle.root, &remote_root)?;
    let remote_exe = remote_root.join(&bundle.build_unit.name);
    debug!("chmod {}", remote_exe.display());
    adb::chmod(env, device, &remote_exe)?;
    Ok((remote_root, remote_exe))
}

fn run_bundle(
    env: &AndroidEnv,
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
    let result = adb::run(env, device, &start_script)?;
    let _ = std::io::stdout().write(result.stdout.as_slice());
    let _ = std::io::stderr().write(result.stderr.as_slice());
    Ok(result)
}
