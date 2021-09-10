use tracing::instrument;

use crate::{
    ios::task::{
        build_buildunits::BuildBuildUnit,
        create_signed_bundles::CreateSignedBundles,
        list_physical_devices::ListPhysicalDevices,
        read_signing_settings::ReadSigningSettings,
        run_on_physical_device::RunOnPhysicalDevice,
        Context,
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
        Context {
            requested,
            devices: None,
            simulators: None,
            build_units: None,
            signing_settings: None,
            build_bundles: None,
        },
    )?;
    Ok(())
}
