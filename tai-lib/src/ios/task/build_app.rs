use crate::{task::Task, TaiResult};

use super::Context;

pub struct BuildApp;

impl Task for BuildApp {
    type Context = Context;

    fn run(&self, mut context: Self::Context) -> TaiResult<Self::Context> {
        // xcodebuild::build(&project_dir, lib_name, data_path_build_app)?;

        Ok(context)
    }
}
