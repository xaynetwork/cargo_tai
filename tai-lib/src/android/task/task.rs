use crate::{common::task::get_project_metadata::GetProjectMetadata, TaiResult};

use super::{BuildBuildUnit, Context, CreateBundles, FindAndroidSdk, ListDevices, RunOnDevices};

pub enum Task {
    FindAndroidSdk(FindAndroidSdk),
    ListDevices(ListDevices),
    BuildBuildUnit(BuildBuildUnit),
    CreateBundles(CreateBundles),
    RunOnDevices(RunOnDevices),
    GetProjectMetadata(GetProjectMetadata),
}

impl crate::common::task::Task<Context> for Task {
    fn run(&self, context: Context) -> TaiResult<Context> {
        match self {
            Task::FindAndroidSdk(task) => task.run(context),
            Task::ListDevices(task) => task.run(context),
            Task::BuildBuildUnit(task) => task.run(context),
            Task::CreateBundles(task) => task.run(context),
            Task::RunOnDevices(task) => task.run(context),
            Task::GetProjectMetadata(task) => task.run(context),
        }
    }
}
