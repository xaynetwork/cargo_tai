pub mod build_build_units;
pub mod context;
pub mod create_bundles;
pub mod find_android_sdk;
pub mod list_devices;
pub mod run_on_devices;
pub mod task;

pub use self::{
    build_build_units::BuildBuildUnit,
    context::{Context, Options},
    create_bundles::CreateBundles,
    find_android_sdk::FindAndroidSdk,
    list_devices::ListDevices,
    run_on_devices::RunOnDevices,
    task::Task,
};

pub struct GetProjectMetadata;

impl crate::task::Task for GetProjectMetadata {
    type Context = Context;

    fn run(&self, mut context: Self::Context) -> crate::TaiResult<Self::Context> {
        let cargo_args = &context.requested.general.compiler.cargo_args;
        let meta = crate::project::ProjectMetadata::from_cargo_args(cargo_args)?;

        context.project_metadata = Some(meta);
        Ok(context)
    }
}
