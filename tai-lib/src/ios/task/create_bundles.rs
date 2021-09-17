use tracing::instrument;

use crate::{
    common::{bundle::create_bundles, task::Task},
    ios::{bundle::bundler::create_bundle, platform::APP_ID},
    TaiResult,
};

use super::Context;

pub struct CreateBundles;

impl Task<Context> for CreateBundles {
    #[instrument(name = "create_bundles", skip(self, context))]
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let built_units = context.take_built_units()?;
        let resources = &context.opts.resources;
        let project_meta = context.project_metadata()?;

        let bundles = create_bundles(built_units, &project_meta.tai_target, |unit, root| {
            create_bundle(unit, root, resources, APP_ID)
        })?;

        context.built_bundles = Some(bundles);

        Ok(context)
    }
}
