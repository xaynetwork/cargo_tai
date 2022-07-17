use std::{
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::bail;
use tracing::{debug, info, instrument};

use crate::{
    android::tools::{adb, AndroidEnv},
    common::{
        bundle::{BuiltBundle, BuiltBundles},
        opts::{BinaryOptions, Options},
        task::Task,
        utils::envs_as_string,
    },
    TaiResult,
};

use super::{list_devices::Devices, Context};

const ANDROID_REMOTE_WORKDIR: &str = "/data/local/tmp/cargo-tai";

#[derive(Debug)]
pub struct RunOnDevices;

impl Task<Context> for RunOnDevices {
    #[instrument(name = "Run On Device(s)", skip_all)]
    fn run(&self, context: Context) -> TaiResult<Context> {
        let android_env: &AndroidEnv = context.get();
        let bundles = context.get::<BuiltBundles>();
        let default = BinaryOptions::default();
        let binary_opt = match context.get::<Options>().binary.as_ref() {
            Some(opts) => opts,
            None => &default,
        };

        context.get::<Devices>().0.iter().try_for_each(|device| {
            bundles.bundles.iter().try_for_each(|bundle| {
                info!("On `{}` run bundle `{}`", device.id, bundle.build_unit.name);
                install_and_run_bundle(android_env, &device.id, bundle, binary_opt)
            })
        })?;
        Ok(context)
    }
}

fn install_and_run_bundle(
    android_env: &AndroidEnv,
    device_id: &str,
    bundle: &BuiltBundle,
    binary_opt: &BinaryOptions,
) -> TaiResult<()> {
    let (remote_root, remote_exe) = install_bundle(android_env, device_id, bundle)?;
    let output = run_bundle(
        android_env,
        device_id,
        binary_opt,
        &remote_root,
        &remote_exe,
    )?;

    adb::rm(android_env, device_id, &remote_root)?;

    match output.status.success() {
        true => {
            info!("Run completed successfully!");
            Ok(())
        }
        false => {
            bail!("Run failed with exit code: {:?}", output.status.code())
        }
    }
}

fn install_bundle(
    android_env: &AndroidEnv,
    device_id: &str,
    bundle: &BuiltBundle,
) -> TaiResult<(PathBuf, PathBuf)> {
    let remote_workdir = PathBuf::from(ANDROID_REMOTE_WORKDIR);
    debug!("Adb mkdir `{}`", remote_workdir.display(),);
    adb::mkdir(android_env, device_id, &remote_workdir)?;
    let remote_root = remote_workdir.join(&bundle.root.file_name().unwrap());
    debug!(
        "Adb sync from `{}` to `{}`",
        bundle.root.display(),
        remote_root.display()
    );
    adb::sync(android_env, device_id, &bundle.root, &remote_root)?;
    let remote_exe = remote_root.join(&bundle.build_unit.name);
    debug!("Adb chmod `{}`", remote_exe.display());
    adb::chmod(android_env, device_id, &remote_exe)?;
    Ok((remote_root, remote_exe))
}

fn run_bundle<R: AsRef<Path>, E: AsRef<Path>>(
    android_env: &AndroidEnv,
    device_id: &str,
    binary_opt: &BinaryOptions,
    remote_root: R,
    remote_exe: E,
) -> TaiResult<std::process::Output> {
    let envs_as_string = binary_opt
        .envs
        .as_ref()
        .map(|envs| envs_as_string(envs))
        .unwrap_or_default();
    let start_script = format!(
        include_str!("../templates/start_script.tmpl"),
        remote_bundle_root = remote_root.as_ref().display(),
        envs = envs_as_string,
        remote_executable = remote_exe.as_ref().display(),
        args = binary_opt.args.as_ref().unwrap_or(&vec![]).join(" ")
    );

    info!("App output:");
    let output = adb::run(android_env, device_id, &start_script)?;
    let _ = std::io::stdout().write(output.stdout.as_slice());
    let _ = std::io::stderr().write(output.stderr.as_slice());
    Ok(output)
}
