use tracing::instrument;

use crate::{
    ios::task::{
        BuildBuildUnit,
        Context,
        CreateSignedBundles,
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
    Runner::execute(
        &[
            Task::BuildBuildUnit(BuildBuildUnit),
            Task::ListPhysicalDevices(ListPhysicalDevices),
            Task::ReadSigningSettings(ReadSigningSettings),
            Task::CreateSignedBundles(CreateSignedBundles),
            Task::RunOnPhysicalDevice(RunOnPhysicalDevice),
        ],
        Context::new(requested),
    )?;
    Ok(())
}
