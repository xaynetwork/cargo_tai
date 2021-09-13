use crate::TaiResult;

pub trait Task {
    type Context;

    fn run(&self, context: Self::Context) -> TaiResult<Self::Context>;
}

pub struct Runner;

impl Runner {
    pub fn execute<T>(tasks: &[T], context: T::Context) -> TaiResult<T::Context>
    where
        T: Task,
    {
        let mut context = context;

        for task in tasks {
            context = task.run(context)?;
        }

        Ok(context)
    }
}
