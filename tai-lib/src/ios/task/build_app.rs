use std::convert::TryFrom;

use crate::{
    common::task::Task,
    ios::tools::xcodebuild::{self, Sdk},
    TaiResult,
};

use super::Context;

const BUILD_DIR: &str = "build";

pub struct BuildApp;

impl Task<Context> for BuildApp {
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let project_meta = context.project_metadata()?;
        let xcode_project = context.xcode_project()?;
        let profile = &project_meta.cargo_opts.profile;
        let sdk = Sdk::try_from(&context.requested.general.compiler.target)?;
        let data_path = project_meta.ios_dir().join(BUILD_DIR);

        xcodebuild::build(
            xcode_project.path(),
            &xcode_project.app_name,
            profile,
            &sdk,
            data_path,
        )?;

        let product = project_meta
            .ios_dir()
            .join(BUILD_DIR)
            .join("Build")
            .join("Products")
            .join(format!("{}-{}", profile.as_str(), sdk.as_str()))
            .join(format!("{}.app", &xcode_project.app_name));

        println!("{} -> {}", product.display(), product.exists());

        context.xcode_product = Some(product);
        Ok(context)
    }
}
