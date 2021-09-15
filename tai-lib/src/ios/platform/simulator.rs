use tracing::instrument;

use crate::{
    common::{
        command::Command,
        options::Options,
        task::{get_project_metadata::GetProjectMetadata, Runner},
    },
    ios::task::{BuildBuildUnit, Context, CreateBundles, ListSimulators, RunOnSimulators, Task},
    TaiResult,
};

use super::tasks_for_build_cmd;

#[instrument(name = "build_and_run", skip(requested))]
pub fn run_command(requested: Options) -> TaiResult<()> {
    match &requested.command {
        Command::Build => {
            Runner::execute(&tasks_for_build_cmd(), Context::from(requested))?;
        }
        _ => {
            Runner::execute(
                &[
                    Task::GetProjectMetadata(GetProjectMetadata),
                    Task::BuildBuildUnit(BuildBuildUnit),
                    Task::ListSimulators(ListSimulators),
                    Task::CreateBundles(CreateBundles),
                    Task::RunOnSimulators(RunOnSimulators),
                ],
                Context::from(requested),
            )?;
        }
    }
    Ok(())
}
