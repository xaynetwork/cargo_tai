use crate::{
    common::task::{
        context::Context, get_project_metadata::GetProjectMetadata, set_bench_arg::SetBenchArg,
    },
    TaiResult,
};

pub mod build_built_units;
pub mod create_bundles;
pub mod get_android_env;
pub mod list_devices;
pub mod run_on_devices;

pub use self::{
    build_built_units::BuildBuiltUnits, create_bundles::CreateBundles,
    get_android_env::GetAndroidEnv, list_devices::ListDevices, run_on_devices::RunOnDevices,
};

pub enum Task {
    GetAndroidEnv(GetAndroidEnv),
    ListDevices(ListDevices),
    BuildBuiltUnits(BuildBuiltUnits),
    CreateBundles(CreateBundles),
    RunOnDevices(RunOnDevices),
    GetProjectMetadata(GetProjectMetadata),
    SetBenchArg(SetBenchArg),
}

impl crate::common::task::Task<Context> for Task {
    fn run(&self, context: Context) -> TaiResult<Context> {
        match self {
            Task::GetAndroidEnv(task) => task.run(context),
            Task::ListDevices(task) => task.run(context),
            Task::BuildBuiltUnits(task) => task.run(context),
            Task::CreateBundles(task) => task.run(context),
            Task::RunOnDevices(task) => task.run(context),
            Task::GetProjectMetadata(task) => task.run(context),
            Task::SetBenchArg(task) => task.run(context),
        }
    }
}
