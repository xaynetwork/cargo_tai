use tracing::instrument;

use crate::{
    ios::task::{
        BuildBuildUnit,
        Context,
        CreateBundles,
        GetProjectMetadata,
        ListSimulators,
        RunOnSimulators,
        Task,
    },
    options::Options,
    task::Runner,
    TaiResult,
};

#[instrument(name = "build_and_run", skip(requested))]
pub fn run_command(requested: Options) -> TaiResult<()> {
    Runner::execute(
        &[
            Task::GetProjectMetadata(GetProjectMetadata),
            Task::BuildBuildUnit(BuildBuildUnit),
            Task::ListSimulators(ListSimulators),
            Task::CreateBundles(CreateBundles),
            Task::RunOnSimulators(RunOnSimulators),
        ],
        Context::new(requested)?,
    )?;
    Ok(())
}
