use std::path::{Path, PathBuf};

use cargo_metadata::{Metadata, MetadataCommand};
use tracing::instrument;

use crate::TaiResult;

pub const CARGO_TAI_TARGET_DIR: &str = "cargo-tai";
pub const IOS_DIR: &str = "ios";
pub const ANDROID_DIR: &str = "android";

pub trait Task {
    type Context;

    fn run(&self, context: Self::Context) -> TaiResult<Self::Context>;
}

pub struct Runner;

impl Runner {
    pub fn execute<T>(tasks: &[T], context: T::Context) -> TaiResult<T::Context>
    where
        T: Task,
    {
        let mut context = context;

        for task in tasks {
            context = task.run(context)?;
        }

        Ok(context)
    }
}

pub struct ProjectMetadata {
    pub meta: Metadata,
}

impl ProjectMetadata {
    pub fn tai_target_dir(&self) -> PathBuf {
        let project_target_dir = &self.meta.target_directory;
        project_target_dir
            .join(CARGO_TAI_TARGET_DIR)
            .into_std_path_buf()
    }

    pub fn ios_dir(&self) -> PathBuf {
        let project_target_dir = &self.meta.target_directory;
        project_target_dir
            .join(CARGO_TAI_TARGET_DIR)
            .join(IOS_DIR)
            .into_std_path_buf()
    }

    pub fn android_dir(&self) -> PathBuf {
        let project_target_dir = &self.meta.target_directory;
        project_target_dir
            .join(CARGO_TAI_TARGET_DIR)
            .join(ANDROID_DIR)
            .into_std_path_buf()
    }
}

pub fn project_metadata<M: AsRef<Path>>(manifest: M) -> TaiResult<ProjectMetadata> {
    let mut cmd = MetadataCommand::new();
    let meta = cmd.manifest_path(manifest.as_ref()).exec()?;
    Ok(ProjectMetadata { meta })
}
