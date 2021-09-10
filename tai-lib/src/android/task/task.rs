use crate::TaiResult;

use super::{BuildBuildUnit, Context, CreateBundles, FindAndroidSdk, ListDevices, RunOnDevices};

pub enum Task {
    FindAndroidSdk(FindAndroidSdk),
    ListDevices(ListDevices),
    BuildBuildUnit(BuildBuildUnit),
    CreateBundles(CreateBundles),
    RunOnDevices(RunOnDevices),
}

impl crate::task::Task for Task {
    type Context = Context;

    fn run(&self, context: Self::Context) -> TaiResult<Self::Context> {
        match self {
            Task::FindAndroidSdk(task) => task.run(context),
            Task::ListDevices(task) => task.run(context),
            Task::BuildBuildUnit(task) => task.run(context),
            Task::CreateBundles(task) => task.run(context),
            Task::RunOnDevices(task) => task.run(context),
        }
    }
}
