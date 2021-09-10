use tracing::instrument;

use crate::{
    ios::task::{
        build_buildunits::BuildBuildUnit,
        create_bundles::CreateBundles,
        list_simulators::ListSimulators,
        run_on_simulators::RunOnSimulators,
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
            Task::ListSimulators(ListSimulators),
            Task::CreateBundles(CreateBundles),
            Task::RunOnSimulators(RunOnSimulators),
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
