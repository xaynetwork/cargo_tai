use crate::{
    common::{
        command::Command,
        opts::Options,
        task::{get_project_metadata::GetProjectMetadata, set_bench_arg::SetBenchArg, Runner},
    },
    ios::{
        platform::tasks_for_build_cmd,
        task::{
            BuildBuiltUnits,
            Context,
            CreateSignedBundles,
            ListPhysicalDevices,
            ReadSigningSettings,
            RunOnPhysicalDevice,
            Task,
        },
    },
    TaiResult,
};

pub fn run_command(requested: Options) -> TaiResult<()> {
    match &requested.command {
        Command::Build => {
            Runner::execute(tasks_for_build_cmd(), Context::from(requested))?;
        }
        _ => {
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
        }
    }
    Ok(())
}
