use tracing::instrument;

use crate::{
    common::{
        command::Command,
        options::Options,
        task::{get_project_metadata::GetProjectMetadata, Runner},
    },
    TaiResult,
};

use super::task::{
    BuildBuiltUnits,
    Context,
    CreateBundles,
    FindAndroidSdk,
    ListDevices,
    RunOnDevices,
    Task,
};

#[instrument(name = "build_and_run", skip(requested))]
pub fn run_command(requested: Options) -> TaiResult<()> {
    match &requested.command {
        Command::Build => {
            unimplemented!()
        }
        _ => {
            Runner::execute(
                &[
                    Task::FindAndroidSdk(FindAndroidSdk),
                    Task::GetProjectMetadata(GetProjectMetadata),
                    Task::BuildBuiltUnits(BuildBuiltUnits),
                    Task::CreateBundles(CreateBundles),
                    Task::ListDevices(ListDevices),
                    Task::RunOnDevices(RunOnDevices),
                ],
                Context::from(requested),
            )?;
        }
    }
    Ok(())
}
