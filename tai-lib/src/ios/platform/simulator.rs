use tracing::instrument;

use crate::{
    command::Command,
    ios::task::{
        BuildApp,
        BuildBuildUnit,
        BuildXCodeTest,
        Context,
        CreateBundles,
        CreateXCodeProject,
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
            Runner::execute(
                &[
                    Task::GetProjectMetadata(GetProjectMetadata),
                    Task::BuildBuildUnit(BuildBuildUnit),
                    Task::CreateXCodeProject(CreateXCodeProject),
                    Task::BuildXCodeTest(BuildXCodeTest),
                    Task::BuildApp(BuildApp),
                ],
                Context::new(requested)?,
            )?;
        }
    }
    Ok(())
}
