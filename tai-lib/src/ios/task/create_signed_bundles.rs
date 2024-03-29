use tracing::instrument;

use crate::{
    common::{
        bundle::{create_bundles, BuiltBundles},
        opts::Options,
        project::ProjectMetadata,
        task::Task,
    },
    ios::bundle::{
        bundler::create_bundle,
        signing::{create_entitlements_file, sign_bundle, SigningSettings},
    },
    TaiResult,
};

use super::{build_built_units::BuiltUnits, Context};

pub struct SignedBuiltBundles(pub BuiltBundles);

pub struct CreateSignedBundles;

impl Task<Context> for CreateSignedBundles {
    #[instrument(name = "create_signed_bundles", skip(self, context))]
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let built_units = context.remove::<BuiltUnits>().0;
        let sig_settings: &SigningSettings = context.get();
        let resources = &context.get::<Options>().resources;
        let project_meta: &ProjectMetadata = context.get();

        let bundles = create_bundles(
            built_units,
            &project_meta.tai_target,
            |unit, bundles_root| create_bundle(unit, bundles_root, resources, &sig_settings.app_id),
        )?;

        let entitlements =
            create_entitlements_file(&project_meta.ios_cache, &sig_settings.entitlements)?;

        bundles
            .bundles
            .iter()
            .try_for_each(|bundle| sign_bundle(bundle, sig_settings, &entitlements))?;

        context.insert(SignedBuiltBundles(bundles));
        Ok(context)
    }
}
