use std::convert::TryFrom;

use crate::{
    common::{project::Profile, task::Task},
    ios::tools::xcodebuild::{Sdk, XCodeBuild},
    TaiResult,
};

use super::Context;

const TEST_BUILD_DIR: &str = "build_test";

pub struct BuildXCodeTest;

impl Task<Context> for BuildXCodeTest {
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let project_meta = context.project_metadata()?;
        let xcode_project = context.xcode_project()?;
        let data_path = project_meta.ios_dir().join(TEST_BUILD_DIR);
        let sdk = Sdk::try_from(&context.options.compiler.target)?;

        XCodeBuild::new()
            .project(xcode_project.path())
            .scheme(&xcode_project.app_name)
            .sdk(sdk)
            .derived_data_path(data_path)
            .build_for_testing()
            .execute()?;

        let product = project_meta
            .ios_dir()
            .join(TEST_BUILD_DIR)
            .join("Build")
            .join("Products")
            .join(format!("{}-{}", Profile::Debug.as_str(), sdk.as_str()))
            .join(format!("{}.app", &xcode_project.app_name))
            .join("PlugIns")
            .join(format!("{}.xctest", &xcode_project.xctest_name()));

        context.xcode_test_product = Some(product);
        Ok(context)
    }
}
