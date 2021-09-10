use tracing::instrument;

use crate::{options::Options, task::Runner, TaiResult};

use super::task::{
    build_buildunits::BuildBuildUnit,
    create_bundles::CreateBundles,
    find_android_sdk::FindAndroidSdk,
    list_devices::ListDevices,
    run_on_devices::RunOnDevices,
    Context,
    Task,
};

#[instrument(name = "build_and_run", skip(requested))]
pub fn run_command(requested: Options) -> TaiResult<()> {
    Runner::execute(
        &[
            Task::FindAndroidSdk(FindAndroidSdk),
            Task::BuildBuildUnit(BuildBuildUnit),
            Task::CreateBundles(CreateBundles),
            Task::ListDevices(ListDevices),
            Task::RunOnDevices(RunOnDevices),
        ],
        Context {
            requested,
            android_sdk: None,
            devices: None,
            build_units: None,
            build_bundles: None,
        },
    )?;
    Ok(())
}
