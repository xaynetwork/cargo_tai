use crate::{
    common::task::{get_project_metadata::GetProjectMetadata, set_bench_arg::SetBenchArg},
    TaiResult,
};

mod build_app;
mod build_built_units;
mod build_xcode_test;
mod context;
mod copy_test_products;
mod create_bundles;
mod create_signed_bundles;
mod create_xcode_project;
mod list_physical_devices;
mod list_simulators;
mod read_signing_settings;
mod run_on_physical_device;
mod run_on_simulators;

pub use self::{
    build_app::BuildApp,
    build_built_units::BuildBuiltUnits,
    build_xcode_test::BuildXCodeTest,
    context::Context,
    copy_test_products::CopyTestProducts,
    create_bundles::CreateBundles,
    create_signed_bundles::CreateSignedBundles,
    create_xcode_project::CreateXCodeProject,
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
    CreateXCodeProject(CreateXCodeProject),
    BuildXCodeTest(BuildXCodeTest),
    BuildApp(BuildApp),
    CopyTestProducts(CopyTestProducts),
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
            Task::CreateXCodeProject(task) => task.run(context),
            Task::BuildXCodeTest(task) => task.run(context),
            Task::BuildApp(task) => task.run(context),
            Task::CopyTestProducts(task) => task.run(context),
            Task::SetBenchArg(task) => task.run(context),
        }
    }
}
