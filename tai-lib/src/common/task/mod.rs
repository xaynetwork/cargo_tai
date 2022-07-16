use tracing::instrument;

use crate::TaiResult;

pub mod context;
pub mod get_project_metadata;
pub mod set_bench_arg;

pub trait Task<C> {
    fn run(&self, context: C) -> TaiResult<C>;
}

pub struct Runner;

impl Runner {
    #[instrument(name = "Task", skip_all)]
    pub fn execute<T, C>(tasks: &[T], context: C) -> TaiResult<C>
    where
        T: Task<C>,
    {
        let mut context = context;

        for task in tasks {
            context = task.run(context)?;
        }

        Ok(context)
    }
}
