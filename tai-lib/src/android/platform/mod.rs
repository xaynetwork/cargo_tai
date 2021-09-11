use tracing::instrument;

use crate::{options::Options, task::Runner, TaiResult};

use super::task::{
    BuildBuildUnit,
    Context,
    CreateBundles,
    FindAndroidSdk,
    GetProjectMetadata,
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
