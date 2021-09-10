use crate::{bundle::BuildBundles, compiler::BuildUnit, options::Options, TaiResult};

use self::{
    build_buildunits::BuildBuildUnit,
    create_bundles::CreateBundles,
    create_signed_bundles::CreateSignedBundles,
    list_physical_devices::ListPhysicalDevices,
    list_simulators::ListSimulators,
    read_signing_settings::ReadSigningSettings,
    run_on_physical_device::RunOnPhysicalDevice,
    run_on_simulators::RunOnSimulators,
};

use super::{bundle::signing::SigningSettings, tools::ios_deploy};

pub mod build_buildunits;
pub mod create_bundles;
pub mod create_signed_bundles;
pub mod list_physical_devices;
pub mod list_simulators;
pub mod read_signing_settings;
pub mod run_on_physical_device;
pub mod run_on_simulators;

pub struct Context {
    pub requested: Options,
    pub devices: Option<Vec<ios_deploy::Device>>,
    pub simulators: Option<Vec<simctl::Device>>,
    pub build_units: Option<Vec<BuildUnit>>,
    pub signing_settings: Option<SigningSettings>,
    pub build_bundles: Option<BuildBundles>,
}

pub enum Task {
    ListPhysicalDevices(ListPhysicalDevices),
    ListSimulators(ListSimulators),
    BuildBuildUnit(BuildBuildUnit),
    ReadSigningSettings(ReadSigningSettings),
    CreateBundles(CreateBundles),
    CreateSignedBundles(CreateSignedBundles),
    RunOnPhysicalDevice(RunOnPhysicalDevice),
    RunOnSimulators(RunOnSimulators),
}

impl crate::task::Task for Task {
    type Context = Context;

    fn run(&self, context: Self::Context) -> TaiResult<Self::Context> {
        match self {
            Task::ListPhysicalDevices(task) => task.run(context),
            Task::ListSimulators(task) => task.run(context),
            Task::BuildBuildUnit(task) => task.run(context),
            Task::ReadSigningSettings(task) => task.run(context),
            Task::CreateBundles(task) => task.run(context),
            Task::CreateSignedBundles(task) => task.run(context),
            Task::RunOnPhysicalDevice(task) => task.run(context),
            Task::RunOnSimulators(task) => task.run(context),
        }
    }
}
