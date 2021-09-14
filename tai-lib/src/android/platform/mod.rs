use tracing::instrument;

use crate::{
    common::{
        options::Options,
        task::{get_project_metadata::GetProjectMetadata, Runner},
    },
    TaiResult,
};

use super::task::{
    BuildBuildUnit,
    Context,
    CreateBundles,
    FindAndroidSdk,
    ListDevices,
    RunOnDevices,
    Task,
};

#[instrument(name = "build_and_run", skip(requested))]
pub fn run_command(requested: Options) -> TaiResult<()> {
    Runner::execute(
        &[
            Task::FindAndroidSdk(FindAndroidSdk),
            Task::GetProjectMetadata(GetProjectMetadata),
            Task::BuildBuildUnit(BuildBuildUnit),
            Task::CreateBundles(CreateBundles),
            Task::ListDevices(ListDevices),
            Task::RunOnDevices(RunOnDevices),
        ],
        Context::new(requested)?,
    )?;
    Ok(())
}
