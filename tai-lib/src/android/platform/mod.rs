use crate::{
    common::{
        opts::Options,
        task::{get_project_metadata::GetProjectMetadata, Runner, set_bench_arg::SetBenchArg},
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
    Runner::execute(
        &[
            Task::FindAndroidSdk(FindAndroidSdk),
            Task::GetProjectMetadata(GetProjectMetadata),
            Task::SetBenchArg(SetBenchArg),
            Task::BuildBuiltUnits(BuildBuiltUnits),
            Task::CreateBundles(CreateBundles),
            Task::ListDevices(ListDevices),
            Task::RunOnDevices(RunOnDevices),
        ],
        Context::from(requested),
    )?;
    Ok(())
}
