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
        let env: &AndroidEnv = context.get();
        let bundles = context.get::<BuiltBundles>();
        let default = BinaryOptions::default();
        let binary_opt = match context.get::<Options>().binary.as_ref() {
            Some(opts) => opts,
            None => &default,
        };

        context.get::<Devices>().0.iter().try_for_each(|device| {
            bundles.bundles.iter().try_for_each(|bundle| {
                info!("On `{}` run bundle `{}`", device.id, bundle.build_unit.name);
                install_and_run_bundle(env, &device.id, bundle, binary_opt)
            })
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

    match result.status.success() {
        true => {
            info!("Run completed successfully!");
            Ok(())
        }
        false => {
            bail!("Run failed with exit code: {:?}", result.status.code())
        }
    }
}

fn install_bundle(
    env: &AndroidEnv,
    device: &str,
    bundle: &BuiltBundle,
) -> TaiResult<(PathBuf, PathBuf)> {
    let remote_workdir = PathBuf::from(ANDROID_REMOTE_WORKDIR);
    adb::mkdir(env, device, &remote_workdir)?;
    let remote_root = remote_workdir.join(&bundle.root.file_name().unwrap());
    debug!(
        "Copy from `{}` to `{}`",
        bundle.root.display(),
        remote_root.display()
    );
    adb::sync(env, device, &bundle.root, &remote_root)?;
    let remote_exe = remote_root.join(&bundle.build_unit.name);
    debug!("chmod `{}`", remote_exe.display());
    adb::chmod(env, device, &remote_exe)?;
    Ok((remote_root, remote_exe))
}

fn run_bundle<R: AsRef<Path>, E: AsRef<Path>>(
    env: &AndroidEnv,
    device: &str,
    binary_opt: &BinaryOptions,
    remote_root: R,
    remote_exe: E,
) -> TaiResult<std::process::Output> {
    let envs_as_string = binary_opt
        .envs
        .as_ref()
        .map(|envs| {
            envs.iter()
                .map(|(key, value)| format!("{}={}", key, value))
                .collect::<Vec<String>>()
                .join(" ")
        })
        .unwrap_or_default();

    let start_script = format!(
        include_str!("../templates/start_script.tmpl"),
        remote_bundle_root = remote_root.as_ref().to_string_lossy(),
        envs = envs_as_string,
        remote_executable = remote_exe.as_ref().to_string_lossy(),
        args = binary_opt.args.as_ref().unwrap_or(&vec![]).join(" ")
    );
    info!("App stdout:");
    let result = adb::run(env, device, &start_script)?;
    let _ = std::io::stdout().write(result.stdout.as_slice());
    let _ = std::io::stderr().write(result.stderr.as_slice());
    Ok(result)
}
