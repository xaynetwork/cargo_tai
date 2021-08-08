use std::{
    convert::TryFrom,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail};
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
    task::{self, GeneralOptions},
    TaiResult,
};

use super::APP_ID;

#[instrument(name = "benches", skip(requested))]
pub fn run_benches(requested: Options) -> TaiResult<()> {
    let build_units = compile_benches(bench_command()?, &requested.general.compiler)?;

    let mut args_with_bench = vec!["--bench".to_string()];
    if let Some(ref args) = requested.general.binary.args {
        args_with_bench.extend_from_slice(args);
    };

    run(
        &requested.mobile_provision,
        build_units,
        &Some(args_with_bench),
        &requested.general.binary.envs,
        &requested.general.binary.resources,
    )
}

#[instrument(name = "tests", skip(requested))]
pub fn run_tests(requested: Options) -> TaiResult<()> {
    let build_units = compile_tests(test_command()?, &requested.general.compiler)?;

    run(
        &requested.mobile_provision,
        build_units,
        &requested.general.binary.args,
        &requested.general.binary.envs,
        &requested.general.binary.resources,
    )
}

#[instrument(name = "run", skip(provision, build_units))]
pub fn run<P>(
    provision: P,
    build_units: Vec<BuildUnit>,
    args: &Option<Vec<String>>,
    envs: &Option<Vec<(String, String)>>,
    resources: &Option<Vec<(String, PathBuf)>>,
) -> TaiResult<()>
where
    P: AsRef<Path>,
{
    let device = ios_deploy::list_device()?.unwrap();
    let sig_settings = find_signing_settings(&device.id, provision.as_ref())?;

    let bundles = create_bundles(build_units, |unit, bundles_root| {
        create_bundle(unit, bundles_root, resources, &sig_settings.app_id)
    })?;
    let entitlements = create_entitlements_file(&bundles.root, &sig_settings.entitlements)?;

    bundles
        .bundles
        .iter()
        .try_for_each(|bundle| sign_bundle(bundle, &sig_settings, &entitlements))?;

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
                "test {} {} failed with: {}",
                APP_ID,
                &bundle_root.as_ref().display(),
                err
            )
        }
    }
}

pub struct Options {
    pub general: GeneralOptions,

    pub mobile_provision: PathBuf,
}

impl TryFrom<task::Options> for Options {
    type Error = anyhow::Error;

    fn try_from(opt: task::Options) -> Result<Self, Self::Error> {
        Ok(Self {
            general: opt.general,
            mobile_provision: opt
                .platform
                .ios_mobile_provision
                .ok_or_else(|| anyhow!("the option mobile_provision is missing"))?,
        })
    }
}
