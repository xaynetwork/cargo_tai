use crate::TaiResult;

pub mod get_project_metadata;

pub trait Task<C> {
    fn run(&self, context: C) -> TaiResult<C>;
}

pub struct Runner;

impl Runner {
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
