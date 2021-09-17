use fs_extra::dir::{copy, CopyOptions};
use std::fs::create_dir_all;
use tracing::{info, instrument};

use crate::{common::task::Task, ios::tools::zip::Zip, TaiResult};

use super::Context;

const PAYLOAD_DIR: &str = "Payload";

pub struct CopyTestProducts;

impl Task<Context> for CopyTestProducts {
    #[instrument(name = "copy_test_products", skip(self, context))]
    fn run(&self, context: Context) -> TaiResult<Context> {
        let xcode_project = context.xcode_project()?;
        let out_dir = &context.build()?.out_dir;
        create_dir_all(&out_dir)?;

        let out_dir = out_dir.canonicalize()?;
        let payload = out_dir.join(PAYLOAD_DIR);
        create_dir_all(&payload)?;

        let opt = CopyOptions {
            overwrite: true,
            ..CopyOptions::default()
        };
        copy(context.xcode_product()?, &payload, &opt)?;

        let zip_file = out_dir.join(format!("{}.ipa", &xcode_project.app_name));
        let mut cmd = Zip::new();
        cmd.current_dir(&out_dir)
            .zip_file(zip_file)
            .file(PAYLOAD_DIR)
            .recurse_paths()
            .move_into_zip_file();
        if context.options.cli.verbose {
            cmd.verbose();
        }

        cmd.execute()?;

        copy(context.xcode_test_product()?, &out_dir, &opt)?;

        info!("test products: {}", out_dir.display());

        Ok(context)
    }
}
