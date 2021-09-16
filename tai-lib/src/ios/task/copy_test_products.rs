use fs_extra::dir::{copy, CopyOptions};
use std::fs::create_dir_all;
use tracing::info;

use crate::{common::task::Task, ios::tools::zip::Zip, TaiResult};

use super::Context;

const PAYLOAD_DIR: &str = "Payload";

pub struct CopyTestProducts;

impl Task<Context> for CopyTestProducts {
    fn run(&self, context: Context) -> TaiResult<Context> {
        let xcode_project = context.xcode_project()?;
        let output_dir = &context.build()?.out_dir.canonicalize()?;

        let payload = output_dir.join(PAYLOAD_DIR);
        create_dir_all(&payload)?;

        let opt = CopyOptions {
            overwrite: true,
            ..CopyOptions::default()
        };
        copy(context.xcode_product()?, &payload, &opt)?;

        let zip_file = output_dir.join(format!("{}.ipa", &xcode_project.app_name));
        Zip::new()
            .current_dir(output_dir)
            .zip_file(zip_file)
            .file(PAYLOAD_DIR)
            .recurse_paths()
            .move_into_zip_file()
            .execute()?;

        copy(context.xcode_test_product()?, &output_dir, &opt)?;

        info!("test products: {}", output_dir.display());

        Ok(context)
    }
}
