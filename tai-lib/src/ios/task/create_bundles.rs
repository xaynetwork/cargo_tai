use tracing::instrument;

use crate::{
    common::{bundle::create_bundles, project::ProjectMetadata, task::Task},
    ios::{bundle::bundler::create_bundle, platform::APP_ID},
    TaiResult,
};

use super::{build_built_units::BuiltUnits, Context};

#[derive(Debug)]
pub struct CreateBundles;

impl Task<Context> for CreateBundles {
    #[instrument(name = "Create Bundles", skip_all)]
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let built_units = context.remove::<BuiltUnits>().0;
        let project_meta: &ProjectMetadata = context.get();

        let bundles = create_bundles(built_units, &project_meta.tai_target, |unit, root| {
            create_bundle(
                unit,
                root,
                APP_ID,
                &project_meta.resources_dir,
                &project_meta.package_graph,
            )
        })?;

        context.insert(bundles);

        Ok(context)
    }
}
