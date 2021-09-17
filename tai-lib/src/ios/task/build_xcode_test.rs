use std::convert::TryFrom;

use tracing::instrument;

use crate::{
    common::{project::Profile, task::Task},
    ios::tools::{xcodebuild::Sdk, XCodeBuild},
    TaiResult,
};

use super::Context;

const TEST_BUILD_DIR: &str = "build_test";

pub struct BuildXCodeTest;

impl Task<Context> for BuildXCodeTest {
    #[instrument(name = "build_xcode_test", skip(self, context))]
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let project_meta = context.project_metadata()?;
        let xcode_project = context.xcode_project()?;
        let data_path = project_meta.ios_working_dir.join(TEST_BUILD_DIR);
        let sdk = Sdk::try_from(&context.opts.compiler.target)?;

        let mut cmd = XCodeBuild::new();
        cmd.project(xcode_project.path())
            .scheme(&xcode_project.app_name)
            .sdk(sdk)
            .derived_data_path(data_path)
            .build_for_testing();
        if context.opts.cli.verbose {
            cmd.verbose();
        }

        cmd.execute()?;

        let product = project_meta
            .ios_working_dir
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
