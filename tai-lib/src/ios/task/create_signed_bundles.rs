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
        let build_units = context.take_build_units()?;
        let sig_settings = context.signing_settings()?;
        let resources = &context.requested.general.binary.resources;

        let bundles = create_bundles(
            build_units,
            context.project_metadata()?,
            |unit, bundles_root| create_bundle(unit, bundles_root, resources, &sig_settings.app_id),
        )?;
        let entitlements = create_entitlements_file(
            &context.project_metadata()?.tai_target_dir(),
            &sig_settings.entitlements,
        )?;

        bundles
            .bundles
            .iter()
            .try_for_each(|bundle| sign_bundle(bundle, sig_settings, &entitlements))?;

        context.build_bundles = Some(bundles);

        Ok(context)
    }
}
