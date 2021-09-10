use crate::{task::Task, TaiResult};

use super::Context;

pub struct BuildXCodeTest;

impl Task for BuildXCodeTest {
    type Context = Context;

    fn run(&self, mut context: Self::Context) -> TaiResult<Self::Context> {
        // xcodebuild::build_for_testing(&project_dir, lib_name, data_path_build_test)?;

        Ok(context)
    }
}
