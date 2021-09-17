use crate::{
    common::{
        command::Command,
        opts::Options,
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
