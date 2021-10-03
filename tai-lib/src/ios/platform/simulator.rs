use crate::{
    common::{
        command::Command,
        opts::Options,
        task::{get_project_metadata::GetProjectMetadata, Runner},
    },
    ios::task::{BuildBuiltUnits, Context, CreateBundles, ListSimulators, RunOnSimulators, Task},
    TaiResult,
};

use super::tasks_for_build_cmd;

pub fn run_command(requested: Options) -> TaiResult<()> {
    match &requested.command {
        Command::Build => {
            Runner::execute(tasks_for_build_cmd(), Context::from(requested))?;
        }
        _ => {
            Runner::execute(
                &[
                    Task::GetProjectMetadata(GetProjectMetadata),
                    Task::BuildBuiltUnits(BuildBuiltUnits),
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
