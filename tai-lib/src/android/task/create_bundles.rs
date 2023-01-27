use crate::{
    android::bundle::create_bundle,
    common::{bundle::create_bundles, opts::Options, project::ProjectMetadata, task::Task},
    TaiResult,
};

use super::{build_built_units::BuiltUnits, Context};

pub struct CreateBundles;

impl Task<Context> for CreateBundles {
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let built_units = context.remove::<BuiltUnits>().0;
        let options = &context.get::<Options>();
        let libraries = &options.libraries;
        let resources = &options.resources;
        let project_meta: &ProjectMetadata = context.get();

        let bundles = create_bundles(built_units, &project_meta.tai_target, |unit, root| {
            create_bundle(unit, root, libraries, resources)
        })?;

        context.insert(bundles);

        Ok(context)
    }
}
