use std::path::Path;

use anyhow::{bail, Error};
use tracing::{info, instrument};

use crate::{
    bundle::create_bundles,
    compiler::{compile_benches, compile_tests, BuildUnit},
    ios::{
        bundle::{
            bundler::create_bundle,
            signing::{create_entitlements_file, find_signing_settings, sign_bundle},
        },
        compiler::{bench_command, test_command},
        tools::ios_deploy,
    },
    task::Options,
    TaiResult,
};

pub const APP_NAME: &'static str = "Dinghy";

#[instrument(name = "benches", skip(requested))]
pub fn run_benches(requested: &Options) -> TaiResult<()> {
    let bench_cmd = bench_command()?;
    let build_units = compile_benches(bench_cmd, requested)?;
    let mut bench_arg = vec!["--bench".to_string()];
    if let Some(ref args) = requested.args {
        bench_arg.extend_from_slice(args);
    };

    run(build_units, &Some(bench_arg), &requested.envs)
}

#[instrument(name = "tests", skip(requested))]
pub fn run_tests(requested: &Options) -> TaiResult<()> {
    let test_cmd = test_command()?;
    let build_units = compile_tests(test_cmd, requested)?;

    run(build_units, &requested.args, &requested.envs)
}

#[instrument(name = "run", skip(build_units))]
pub fn run(
    build_units: Vec<BuildUnit>,
    args: &Option<Vec<String>>,
    envs: &Option<Vec<(String, String)>>,
) -> TaiResult<()> {
    let device = ios_deploy::list_device()?.unwrap();
    let sig_settings = find_signing_settings(&device.id)?;

    let bundles = create_bundles(build_units, |unit, root| {
        create_bundle(unit, root, &sig_settings.app_id)
    })?;
    let entitlements = create_entitlements_file(&bundles.root, &sig_settings.entitlements)?;

    bundles
        .bundles
        .iter()
        .map(|bundle| sign_bundle(&bundle, &sig_settings, &entitlements))
        .collect::<Result<(), Error>>()?;

    bundles
        .bundles
        .iter()
        .map(|bundle| install_and_launch(&bundle.root, args, envs))
        .collect()
}

#[instrument(name = "install_launch", skip(bundle_root))]
fn install_and_launch<P>(
    bundle_root: P,
    args: &Option<Vec<String>>,
    envs: &Option<Vec<(String, String)>>,
) -> TaiResult<()>
where
    P: AsRef<Path>,
{
    match ios_deploy::launch_app(&bundle_root, args, envs) {
        Ok(_) => {
            info!("test result ok");
            Ok(())
        }
        Err(err) => {
            bail!(
                "test {} {:?} failed with: {}",
                APP_NAME,
                &bundle_root.as_ref(),
                err
            )
        }
    }
}
