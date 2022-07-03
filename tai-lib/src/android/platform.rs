use crate::{
    common::{
        opts::Options,
        task::{
            context::Context,
            get_project_metadata::GetProjectMetadata,
            set_bench_arg::SetBenchArg,
            Runner,
        },
    },
    TaiResult,
};

use super::task::{BuildBuiltUnits, CreateBundles, GetAndroidEnv, ListDevices, RunOnDevices, Task};

pub fn run_command(requested: Options) -> TaiResult<()> {
    Runner::execute(
        &[
            Task::GetAndroidEnv(GetAndroidEnv),
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
