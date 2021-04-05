use std::path::Path;

use anyhow::{bail, Error};
use tracing::{info, instrument};

use crate::{
    bundle::create_bundles,
    compiler::compile_tests,
    ios::{
        bundle::{
            bundler::create_bundle,
            signing::{create_entitlements_file, find_signing_settings, sign_bundle},
        },
        compiler::test_command,
        tools::ios_deploy,
    },
    task::Options,
    TaiResult,
};

pub const APP_NAME: &'static str = "Dinghy";

#[instrument(name = "test", skip(requested))]
pub fn run_test(requested: &Options) -> TaiResult<()> {
    let test_cmd = test_command()?;
    let build_units = compile_tests(test_cmd, requested)?;

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
        .map(|bundle| install_and_launch(&bundle.root, &[], &[]))
        .collect()
}

#[instrument(name = "run", skip(bundle_root, args, envs))]
fn install_and_launch<P>(bundle_root: P, args: &[&str], envs: &[&str]) -> TaiResult<()>
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
