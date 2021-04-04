use anyhow::{anyhow, bail};
use std::{io::Write, path::PathBuf};
use tools::ANDROID_APP_WORK_DIR;
use tracing::debug;

use crate::{
    android::{bundle::bundler::create_bundles, tools},
    compiler::compile_tests,
    task::Options,
    TaiResult,
};

use super::{bundle::BuildBundle, compiler::test_command};

pub fn run_test(requested: &Options) -> TaiResult<()> {
    let test_cmd = test_command(requested)?;
    let build_units = compile_tests(test_cmd, requested)?;
    let devices = tools::devices()?
        .pop()
        .ok_or(anyhow!("no android device available"))?;

    let bundles = create_bundles(build_units)?;

    bundles
        .bundles
        .iter()
        .map(|bundle| install_and_launch(&devices.id, &bundle, &[], &[]))
        .collect()
}

fn install_and_launch(
    device: &str,
    bundle: &BuildBundle,
    args: &[&str],
    envs: &[&str],
) -> TaiResult<()> {
    let work_dir = PathBuf::from(ANDROID_APP_WORK_DIR);
    tools::mkdir(device, &work_dir)?;

    let remote_root = work_dir.join(&bundle.root.file_name().unwrap());
    debug!("copy from: {:?} to: {:?}", bundle.root, remote_root);
    tools::sync(device, &bundle.root, &remote_root)?;
    let remote_exe = remote_root.join(&bundle.build_unit.name);
    // debug!("chmod {:?}", remote_exe);
    // tools::chmod(device, &remote_exe)?;

    let start_script = format!(
        include_str!("../templates/start_script.tmpl"),
        remote_bundle_root = remote_root.to_string_lossy(),
        envs = envs.join(" "),
        remote_executable = remote_exe.to_string_lossy(),
        args = args.join(" ")
    );

    let result = tools::run(device, &start_script)?;
    let _ = std::io::stdout().write(result.stdout.as_slice());
    let _ = std::io::stderr().write(result.stderr.as_slice());

    tools::rm(device, &remote_root)?;

    if result.status.success() {
        Ok(())
    } else {
        bail!("test failed")
    }
}
