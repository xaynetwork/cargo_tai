use std::path::{Path, PathBuf};

use anyhow::bail;
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

pub const APP_NAME: &str = "Dinghy";

#[instrument(name = "benches", skip(requested))]
pub fn run_benches(requested: &Options) -> TaiResult<()> {
    let build_units = compile_benches(bench_command()?, requested)?;

    let mut args_with_bench = vec!["--bench".to_string()];
    if let Some(ref args) = requested.args {
        args_with_bench.extend_from_slice(args);
    };

    run(
        build_units,
        &Some(args_with_bench),
        &requested.envs,
        &requested.resources,
    )
}

#[instrument(name = "tests", skip(requested))]
pub fn run_tests(requested: &Options) -> TaiResult<()> {
    let build_units = compile_tests(test_command()?, requested)?;

    run(
        build_units,
        &requested.args,
        &requested.envs,
        &requested.resources,
    )
}

#[instrument(name = "run", skip(build_units))]
pub fn run(
    build_units: Vec<BuildUnit>,
    args: &Option<Vec<String>>,
    envs: &Option<Vec<(String, String)>>,
    resources: &Option<Vec<(String, PathBuf)>>,
) -> TaiResult<()> {
    let device = ios_deploy::list_device()?.unwrap();
    let sig_settings = find_signing_settings(&device.id)?;

    let bundles = create_bundles(build_units, |unit, bundles_root| {
        create_bundle(unit, bundles_root, resources, &sig_settings.app_id)
    })?;
    let entitlements = create_entitlements_file(&bundles.root, &sig_settings.entitlements)?;

    bundles
        .bundles
        .iter()
        .try_for_each(|bundle| sign_bundle(&bundle, &sig_settings, &entitlements))?;

    bundles
        .bundles
        .iter()
        .try_for_each(|bundle| install_and_launch(&bundle.root, args, envs))
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
