use tracing::instrument;

use crate::{
    common::{opts::Options, project::ProjectMetadata, task::Task},
    TaiResult,
};

use super::context::Context;

pub struct GetProjectMetadata;

impl Task<Context> for GetProjectMetadata {
    #[instrument(name = "Setup", skip_all)]
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let cargo_args = &context.get::<Options>().compiler.cargo_args;
        let meta = ProjectMetadata::from_cargo_args(cargo_args)?;

        context.insert(meta);
        Ok(context)
    }
}
