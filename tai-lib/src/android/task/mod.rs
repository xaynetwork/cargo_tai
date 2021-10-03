use crate::{common::task::get_project_metadata::GetProjectMetadata, TaiResult};

pub mod build_android_test;
pub mod build_built_units;
pub mod context;
pub mod create_android_project;
pub mod create_bundles;
pub mod find_android_sdk;
pub mod list_devices;
pub mod run_on_devices;

pub use self::{
    build_built_units::BuildBuiltUnits,
    context::Context,
    create_android_project::CreateAndroidProject,
    create_bundles::CreateBundles,
    find_android_sdk::FindAndroidSdk,
    list_devices::ListDevices,
    run_on_devices::RunOnDevices,
};

pub enum Task {
    FindAndroidSdk(FindAndroidSdk),
    ListDevices(ListDevices),
    BuildBuiltUnits(BuildBuiltUnits),
    CreateBundles(CreateBundles),
    RunOnDevices(RunOnDevices),
    GetProjectMetadata(GetProjectMetadata),
    // BuildUniversalBuilt(),
    CreateAndroidProject(CreateAndroidProject),
    // BuildAndroidTest(),
}

impl crate::common::task::Task<Context> for Task {
    fn run(&self, context: Context) -> TaiResult<Context> {
        match self {
            Task::FindAndroidSdk(task) => task.run(context),
            Task::ListDevices(task) => task.run(context),
            Task::BuildBuiltUnits(task) => task.run(context),
            Task::CreateBundles(task) => task.run(context),
            Task::RunOnDevices(task) => task.run(context),
            Task::GetProjectMetadata(task) => task.run(context),
            Task::CreateAndroidProject(task) => task.run(context),
        }
    }
}
