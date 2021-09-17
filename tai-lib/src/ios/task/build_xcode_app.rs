use std::convert::TryFrom;

use tracing::instrument;

use crate::{
    common::task::Task,
    ios::tools::xcodebuild::{Sdk, XCodeBuild},
    TaiResult,
};

use super::Context;

const BUILD_DIR: &str = "build";

pub struct BuildXCodeApp;

impl Task<Context> for BuildXCodeApp {
    #[instrument(name = "build_xcode_app", skip(self, context))]
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let project_meta = context.project_metadata()?;
        let xcode_project = context.xcode_project()?;
        let profile = project_meta.cargo_opts.profile;
        let sdk = Sdk::try_from(&context.options.compiler.target)?;
        let data_path = project_meta.ios_dir().join(BUILD_DIR);

        let mut cmd = XCodeBuild::new();
        cmd.project(xcode_project.path())
            .scheme(&xcode_project.app_name)
            .profile(profile)
            .sdk(sdk)
            .derived_data_path(data_path);
        if context.signing_settings().is_ok() {
            cmd.allow_provisioning_updates();
        }
        if context.options.cli.verbose {
            cmd.verbose();
        }

        cmd.execute()?;

        let product = project_meta
            .ios_dir()
            .join(BUILD_DIR)
            .join("Build")
            .join("Products")
            .join(format!("{}-{}", profile.as_str(), sdk.as_str()))
            .join(format!("{}.app", &xcode_project.app_name));

        context.xcode_product = Some(product);
        Ok(context)
    }
}
