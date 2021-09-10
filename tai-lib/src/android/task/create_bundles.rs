use crate::{android::bundle::create_bundle, bundle::create_bundles, task::Task, TaiResult};

use super::Context;

pub struct CreateBundles;

impl Task for CreateBundles {
    type Context = Context;

    fn run(&self, mut context: Self::Context) -> TaiResult<Self::Context> {
        let build_units = context.take_build_units()?;
        let resources = &context.requested.general.binary.resources;

        let bundles = create_bundles(build_units, |unit, root| {
            create_bundle(unit, root, resources)
        })?;

        context.build_bundles = Some(bundles);

        Ok(context)
    }
}
