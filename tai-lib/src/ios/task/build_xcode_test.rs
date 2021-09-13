use crate::{ios::tools::xcodebuild, task::Task, TaiResult};

use super::Context;

const TEST_BUILD_DIR: &str = "build_test";

pub struct BuildXCodeTest;

impl Task for BuildXCodeTest {
    type Context = Context;

    fn run(&self, context: Self::Context) -> TaiResult<Self::Context> {
        let project_meta = context.project_metadata()?;
        let xcode_project = context.xcode_project()?;
        let data_path = project_meta.ios_dir().join(TEST_BUILD_DIR);

        xcodebuild::build_for_testing(xcode_project.path(), &xcode_project.app_name, &data_path)?;

        Ok(context)
    }
}
