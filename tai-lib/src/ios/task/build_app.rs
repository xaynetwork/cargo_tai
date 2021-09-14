use crate::{
    common::task::Task,
    ios::tools::xcodebuild::{self},
    TaiResult,
};

use super::Context;

const BUILD_DIR: &str = "build";

pub struct BuildApp;

impl Task<Context> for BuildApp {
    fn run(&self, context: Context) -> TaiResult<Context> {
        let project_meta = context.project_metadata()?;
        let xcode_project = context.xcode_project()?;
        let data_path = project_meta.ios_dir().join(BUILD_DIR);

        xcodebuild::build(
            xcode_project.path(),
            &xcode_project.app_name,
            &project_meta.cargo_opts.profile,
            data_path,
        )?;

        Ok(context)
    }
}
