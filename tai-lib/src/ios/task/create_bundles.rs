use crate::{
    common::{bundle::create_bundles, task::Task},
    ios::{bundle::bundler::create_bundle, platform::APP_ID},
    TaiResult,
};

use super::Context;

pub struct CreateBundles;

impl Task<Context> for CreateBundles {
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let build_units = context.take_build_units()?;
        let resources = &context.binary()?.resources;

        let bundles = create_bundles(build_units, context.project_metadata()?, |unit, root| {
            create_bundle(unit, root, resources, APP_ID)
        })?;

        context.build_bundles = Some(bundles);

        Ok(context)
    }
}
