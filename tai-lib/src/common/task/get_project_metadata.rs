use crate::{
    android::task::Context as AndroidContext,
    common::{project::ProjectMetadata, task::Task},
    ios::task::Context as IosContext,
    TaiResult,
};

pub struct GetProjectMetadata;

impl Task<IosContext> for GetProjectMetadata {
    fn run(&self, mut context: IosContext) -> TaiResult<IosContext> {
        let cargo_args = &context.requested.general.compiler.cargo_args;
        let meta = ProjectMetadata::from_cargo_args(cargo_args)?;

        context.project_metadata = Some(meta);
        Ok(context)
    }
}

impl Task<AndroidContext> for GetProjectMetadata {
    fn run(&self, mut context: AndroidContext) -> TaiResult<AndroidContext> {
        let cargo_args = &context.requested.general.compiler.cargo_args;
        let meta = ProjectMetadata::from_cargo_args(cargo_args)?;

        context.project_metadata = Some(meta);
        Ok(context)
    }
}
