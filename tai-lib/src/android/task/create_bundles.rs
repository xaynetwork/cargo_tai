use crate::{
    android::bundle::create_bundle,
    common::{bundle::create_bundles, task::Task},
    TaiResult,
};

use super::Context;

pub struct CreateBundles;

impl Task<Context> for CreateBundles {
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let build_units = context.take_build_units()?;
        let resources = &context.requested.general.binary.resources;

        let bundles = create_bundles(build_units, context.project_metadata()?, |unit, root| {
            create_bundle(unit, root, resources)
        })?;

        context.build_bundles = Some(bundles);

        Ok(context)
    }
}
