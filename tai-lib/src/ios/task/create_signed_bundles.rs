use crate::{
    common::{bundle::create_bundles, task::Task},
    ios::bundle::{
        bundler::create_bundle,
        signing::{create_entitlements_file, sign_bundle},
    },
    TaiResult,
};

use super::Context;

pub struct CreateSignedBundles;

impl Task<Context> for CreateSignedBundles {
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let built_units = context.take_built_units()?;
        let sig_settings = context.signing_settings()?;
        let resources = &context.binary()?.resources;

        let bundles = create_bundles(
            built_units,
            context.project_metadata()?,
            |unit, bundles_root| create_bundle(unit, bundles_root, resources, &sig_settings.app_id),
        )?;
        let entitlements = create_entitlements_file(
            &context.project_metadata()?.ios_dir(),
            &sig_settings.entitlements,
        )?;

        bundles
            .bundles
            .iter()
            .try_for_each(|bundle| sign_bundle(bundle, sig_settings, &entitlements))?;

        context.built_bundles = Some(bundles);

        Ok(context)
    }
}
