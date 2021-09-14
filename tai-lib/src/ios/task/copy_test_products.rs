use fs_extra::dir::{copy, remove, CopyOptions};
use std::fs::create_dir_all;
use tracing::info;

use crate::{common::task::Task, ios::tools::zip::zip, TaiResult};

use super::Context;

pub struct CopyTestProducts;

impl Task<Context> for CopyTestProducts {
    fn run(&self, context: Context) -> TaiResult<Context> {
        let project_meta = context.project_metadata()?;
        let xcode_project = context.xcode_project()?;

        // make it a configurable
        let output_dir = project_meta.ios_dir().join("out");

        let payload = output_dir.join("Payload");
        create_dir_all(&payload)?;

        let opt = CopyOptions::new();
        copy(context.xcode_product()?, &payload, &opt)?;

        zip(
            output_dir.join(format!("{}.ipa", &xcode_project.app_name)),
            &payload,
        )?;
        remove(payload)?;

        copy(context.xcode_test_product()?, &output_dir, &opt)?;

        info!("test products: {}", output_dir.display());

        Ok(context)
    }
}
