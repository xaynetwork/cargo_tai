use crate::{task::Task, TaiResult};

use super::Context;

pub struct CreateXCodeProject;

impl Task for CreateXCodeProject {
    type Context = Context;

    fn run(&self, mut context: Self::Context) -> TaiResult<Self::Context> {
        // xcodegen::generate(&spec, &project_dir)?;

        Ok(context)
    }
}
