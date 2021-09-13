use super::task::{
    BuildApp,
    BuildBuildUnit,
    BuildXCodeTest,
    CreateXCodeProject,
    GetProjectMetadata,
    Task,
};

pub mod physical;
pub mod simulator;

pub const APP_ID: &str = "cargo-tai";

fn tasks_for_build_cmd() -> [Task; 5] {
    [
        Task::GetProjectMetadata(GetProjectMetadata),
        Task::BuildBuildUnit(BuildBuildUnit),
        // Task::ReadSigningSettings(ReadSigningSettings), we need the team id later + bundle id
        Task::CreateXCodeProject(CreateXCodeProject),
        Task::BuildApp(BuildApp),
        Task::BuildXCodeTest(BuildXCodeTest),
    ]
}
