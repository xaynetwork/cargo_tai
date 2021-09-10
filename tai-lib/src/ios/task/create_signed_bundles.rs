use anyhow::anyhow;

use crate::{
    bundle::create_bundles,
    ios::bundle::{
        bundler::create_bundle,
        signing::{create_entitlements_file, sign_bundle},
    },
    task::Task,
    TaiResult,
};

use super::Context;

pub struct CreateSignedBundles;

impl Task for CreateSignedBundles {
    type Context = Context;

    fn run(&self, mut context: Self::Context) -> TaiResult<Self::Context> {
        let build_units = context
            .build_units
            .take()
            .ok_or_else(|| anyhow!("no units to bundle"))?;
        let sig_settings = context
            .signing_settings
            .as_ref()
            .ok_or_else(|| anyhow!("no signing settings"))?;
        let resources = &context.requested.general.binary.resources;

        let bundles = create_bundles(build_units, |unit, bundles_root| {
            create_bundle(unit, bundles_root, resources, &sig_settings.app_id)
        })?;
        let entitlements = create_entitlements_file(&bundles.root, &sig_settings.entitlements)?;

        bundles
            .bundles
            .iter()
            .try_for_each(|bundle| sign_bundle(bundle, sig_settings, &entitlements))?;

        context.build_bundles = Some(bundles);

        Ok(context)
    }
}