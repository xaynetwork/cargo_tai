use std::{
    convert::TryFrom,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail};
use tracing::{info, instrument};

use crate::{
    bundle::create_bundles,
    ios::{
        bundle::{
            bundler::create_bundle,
            signing::{create_entitlements_file, find_signing_settings, sign_bundle},
        },
        platform::compile_build_units,
        tools::ios_deploy,
    },
    task::{self, BinaryOptions, GeneralOptions},
    TaiResult,
};

use super::APP_ID;

#[instrument(name = "build_and_run", skip(requested))]
pub fn run_task(requested: Options) -> TaiResult<()> {
    let build_units = compile_build_units(&requested.general)?;

    let devices = ios_deploy::list_device()?;
    let device = devices
        .first()
        .ok_or_else(|| anyhow!("no iOS device available"))?;
    let sig_settings = find_signing_settings(&device.id, &requested.mobile_provision)?;

    let bundles = create_bundles(build_units, |unit, bundles_root| {
        create_bundle(
            unit,
            bundles_root,
            &requested.general.binary.resources,
            &sig_settings.app_id,
        )
    })?;
    let entitlements = create_entitlements_file(&bundles.root, &sig_settings.entitlements)?;

    bundles
        .bundles
        .iter()
        .try_for_each(|bundle| sign_bundle(bundle, &sig_settings, &entitlements))?;

    bundles
        .bundles
        .iter()
        .try_for_each(|bundle| install_and_launch(&bundle.root, &requested.general.binary))
}

#[instrument(name = "install_launch", skip(bundle_root))]
fn install_and_launch<P>(bundle_root: P, binary_opt: &BinaryOptions) -> TaiResult<()>
where
    P: AsRef<Path>,
{
    match ios_deploy::launch_app(&bundle_root, &binary_opt.args, &binary_opt.envs) {
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
