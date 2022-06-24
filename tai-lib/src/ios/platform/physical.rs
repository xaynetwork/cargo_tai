use crate::{
    common::{
        opts::Options,
        task::{get_project_metadata::GetProjectMetadata, set_bench_arg::SetBenchArg, Runner},
    },
    ios::task::{
        BuildBuiltUnits,
        Context,
        CreateSignedBundles,
        ListPhysicalDevices,
        ReadSigningSettings,
        RunOnPhysicalDevice,
        Task,
    },
    TaiResult,
};

pub fn run_command(requested: Options) -> TaiResult<()> {
    Runner::execute(
        &[
            Task::GetProjectMetadata(GetProjectMetadata),
            Task::SetBenchArg(SetBenchArg),
            Task::BuildBuiltUnits(BuildBuiltUnits),
            Task::ListPhysicalDevices(ListPhysicalDevices),
            Task::ReadSigningSettings(ReadSigningSettings),
            Task::CreateSignedBundles(CreateSignedBundles),
            Task::RunOnPhysicalDevice(RunOnPhysicalDevice),
        ],
        Context::from(requested),
    )?;
    Ok(())
}
