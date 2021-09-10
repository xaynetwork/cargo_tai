use tracing::instrument;

use crate::{
    ios::task::{BuildBuildUnit, Context, CreateBundles, ListSimulators, RunOnSimulators, Task},
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
        Context::new(requested),
    )?;
    Ok(())
}
