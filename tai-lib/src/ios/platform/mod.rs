use crate::common::task::get_project_metadata::GetProjectMetadata;

use super::task::{
    BuildBuiltUnits,
    BuildXCodeApp,
    BuildXCodeTest,
    CopyTestProducts,
    CreateXCodeProject,
    Task,
};

pub mod physical;
pub mod simulator;

pub const APP_ID: &str = "cargo-tai";

fn tasks_for_build_cmd() -> [Task; 6] {
    [
        Task::GetProjectMetadata(GetProjectMetadata),
        Task::BuildBuiltUnits(BuildBuiltUnits),
        // Task::ReadSigningSettings(ReadSigningSettings), we need the team id later + bundle id
        Task::CreateXCodeProject(CreateXCodeProject),
        Task::BuildXCodeApp(BuildXCodeApp),
        Task::BuildXCodeTest(BuildXCodeTest),
        Task::CopyTestProducts(CopyTestProducts),
    ]
}
