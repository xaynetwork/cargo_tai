use tracing::{info, instrument};

use crate::{
    command::Command,
    ios::task::{
        BuildApp,
        BuildBuildUnit,
        BuildXCodeTest,
        Context,
        CreateSignedBundles,
        CreateXCodeProject,
        GetProjectMetadata,
        ListPhysicalDevices,
        ReadSigningSettings,
        RunOnPhysicalDevice,
        Task,
    },
    options::Options,
    task::Runner,
    TaiResult,
};

#[instrument(name = "build_and_run", skip(requested))]
pub fn run_command(requested: Options) -> TaiResult<()> {
    info!("run_command");

    match &requested.general.command {
        Command::Bench | Command::Test | Command::Benches | Command::Tests => {
            Runner::execute(
                &[
                    Task::GetProjectMetadata(GetProjectMetadata),
                    Task::BuildBuildUnit(BuildBuildUnit),
                    Task::ListPhysicalDevices(ListPhysicalDevices),
                    Task::ReadSigningSettings(ReadSigningSettings),
                    Task::CreateSignedBundles(CreateSignedBundles),
                    Task::RunOnPhysicalDevice(RunOnPhysicalDevice),
                ],
                Context::new(requested)?,
            )?;
        }
        Command::Build => {
            Runner::execute(
                &[
                    Task::GetProjectMetadata(GetProjectMetadata),
                    Task::BuildBuildUnit(BuildBuildUnit),
                    Task::ReadSigningSettings(ReadSigningSettings),
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
