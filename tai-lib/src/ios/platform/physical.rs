use tracing::{info, instrument};

use crate::{
    command::Command,
    ios::{
        platform::tasks_for_build_cmd,
        task::{
            BuildBuildUnit,
            Context,
            CreateSignedBundles,
            GetProjectMetadata,
            ListPhysicalDevices,
            ReadSigningSettings,
            RunOnPhysicalDevice,
            Task,
        },
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
            Runner::execute(&tasks_for_build_cmd(), Context::new(requested)?)?;
        }
    }
    Ok(())
}
