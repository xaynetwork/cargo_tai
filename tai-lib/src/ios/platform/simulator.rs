use crate::{
    common::{
        opts::Options,
        task::{get_project_metadata::GetProjectMetadata, set_bench_arg::SetBenchArg, Runner},
    },
    ios::task::{BuildBuiltUnits, Context, CreateBundles, ListSimulators, RunOnSimulators, Task},
    TaiResult,
};

pub fn run_command(requested: Options) -> TaiResult<()> {
    Runner::execute(
        &[
            Task::GetProjectMetadata(GetProjectMetadata),
            Task::SetBenchArg(SetBenchArg),
            Task::BuildBuiltUnits(BuildBuiltUnits),
            Task::ListSimulators(ListSimulators),
            Task::CreateBundles(CreateBundles),
            Task::RunOnSimulators(RunOnSimulators),
        ],
        Context::from(requested),
    )?;
    Ok(())
}
