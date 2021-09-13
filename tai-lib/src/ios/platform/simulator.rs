use tracing::instrument;

use crate::{
    command::Command,
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

use super::tasks_for_build_cmd;

#[instrument(name = "build_and_run", skip(requested))]
pub fn run_command(requested: Options) -> TaiResult<()> {
    match &requested.general.command {
        Command::Bench | Command::Test | Command::Benches | Command::Tests => {
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
        }
        Command::Build => {
            Runner::execute(&tasks_for_build_cmd(), Context::new(requested)?)?;
        }
    }
    Ok(())
}
