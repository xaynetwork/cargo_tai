use crate::{
    android::bundle::create_bundle,
    common::{bundle::create_bundles, task::Task},
    TaiResult,
};

use super::Context;

pub struct CreateBundles;

impl Task<Context> for CreateBundles {
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let built_units = context.take_built_units()?;
        let resources = &context.opts.resources;
        let project_meta = context.project_metadata()?;

        let bundles = create_bundles(built_units, &project_meta.tai_target, |unit, root| {
            create_bundle(unit, root, resources)
        })?;

        context.built_bundles = Some(bundles);

        Ok(context)
    }
}
