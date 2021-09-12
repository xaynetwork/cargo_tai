use crate::TaiResult;

use super::{
    BuildApp,
    BuildBuildUnit,
    BuildXCodeTest,
    Context,
    CreateBundles,
    CreateSignedBundles,
    CreateXCodeProject,
    GetProjectMetadata,
    ListPhysicalDevices,
    ListSimulators,
    ReadSigningSettings,
    RunOnPhysicalDevice,
    RunOnSimulators,
};

pub enum Task {
    ListPhysicalDevices(ListPhysicalDevices),
    ListSimulators(ListSimulators),
    BuildBuildUnit(BuildBuildUnit),
    ReadSigningSettings(ReadSigningSettings),
    CreateBundles(CreateBundles),
    CreateSignedBundles(CreateSignedBundles),
    RunOnPhysicalDevice(RunOnPhysicalDevice),
    RunOnSimulators(RunOnSimulators),
    GetProjectMetadata(GetProjectMetadata),
    CreateXCodeProject(CreateXCodeProject),
    BuildXCodeTest(BuildXCodeTest),
    BuildApp(BuildApp),
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
            Task::GetProjectMetadata(task) => task.run(context),
            Task::CreateXCodeProject(task) => task.run(context),
            Task::BuildXCodeTest(task) => task.run(context),
            Task::BuildApp(task) => task.run(context),
        }
    }
}
