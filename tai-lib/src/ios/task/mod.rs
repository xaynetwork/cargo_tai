use crate::{
    common::task::{
        context::Context,
        get_project_metadata::GetProjectMetadata,
        set_bench_arg::SetBenchArg,
    },
    TaiResult,
};

mod build_built_units;
mod create_bundles;
mod create_signed_bundles;
mod list_physical_devices;
mod list_simulators;
mod read_signing_settings;
mod run_on_physical_device;
mod run_on_simulators;

pub use self::{
    build_built_units::BuildBuiltUnits,
    create_bundles::CreateBundles,
    create_signed_bundles::CreateSignedBundles,
    list_physical_devices::ListPhysicalDevices,
    list_simulators::ListSimulators,
    read_signing_settings::ReadSigningSettings,
    run_on_physical_device::RunOnPhysicalDevice,
    run_on_simulators::RunOnSimulators,
};

pub enum Task {
    ListPhysicalDevices(ListPhysicalDevices),
    ListSimulators(ListSimulators),
    BuildBuiltUnits(BuildBuiltUnits),
    ReadSigningSettings(ReadSigningSettings),
    CreateBundles(CreateBundles),
    CreateSignedBundles(CreateSignedBundles),
    RunOnPhysicalDevice(RunOnPhysicalDevice),
    RunOnSimulators(RunOnSimulators),
    GetProjectMetadata(GetProjectMetadata),
    SetBenchArg(SetBenchArg),
}

impl crate::common::task::Task<Context> for Task {
    fn run(&self, context: Context) -> TaiResult<Context> {
        match self {
            Task::ListPhysicalDevices(task) => task.run(context),
            Task::ListSimulators(task) => task.run(context),
            Task::BuildBuiltUnits(task) => task.run(context),
            Task::ReadSigningSettings(task) => task.run(context),
            Task::CreateBundles(task) => task.run(context),
            Task::CreateSignedBundles(task) => task.run(context),
            Task::RunOnPhysicalDevice(task) => task.run(context),
            Task::RunOnSimulators(task) => task.run(context),
            Task::GetProjectMetadata(task) => task.run(context),
            Task::SetBenchArg(task) => task.run(context),
        }
    }
}
