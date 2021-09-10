use crate::{bundle::BuildBundles, compiler::BuildUnit, options::Options};

use self::{
    build_buildunits::BuildBuildUnit,
    create_bundles::CreateBundles,
    find_android_sdk::FindAndroidSdk,
    list_devices::ListDevices,
    run_on_devices::RunOnDevices,
};

use super::tools::{adb::Device, AndroidSdk};

pub mod build_buildunits;
pub mod create_bundles;
pub mod find_android_sdk;
pub mod list_devices;
pub mod run_on_devices;

pub struct Context {
    pub requested: Options,
    pub android_sdk: Option<AndroidSdk>,
    pub devices: Option<Vec<Device>>,
    pub build_units: Option<Vec<BuildUnit>>,
    pub build_bundles: Option<BuildBundles>,
}

pub enum Task {
    FindAndroidSdk(FindAndroidSdk),
    ListDevices(ListDevices),
    BuildBuildUnit(BuildBuildUnit),
    CreateBundles(CreateBundles),
    RunOnDevices(RunOnDevices),
}

impl crate::task::Task for Task {
    type Context = Context;

    fn run(&self, context: Self::Context) -> crate::TaiResult<Self::Context> {
        match self {
            Task::FindAndroidSdk(task) => task.run(context),
            Task::ListDevices(task) => task.run(context),
            Task::BuildBuildUnit(task) => task.run(context),
            Task::CreateBundles(task) => task.run(context),
            Task::RunOnDevices(task) => task.run(context),
        }
    }
}
