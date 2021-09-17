use crate::{
    android::task::Context as AndroidContext,
    common::{command::Command, opts::Options, task::Task},
    ios::task::Context as IosContext,
    TaiResult,
};

pub struct SetBenchArg;

impl Task<IosContext> for SetBenchArg {
    fn run(&self, mut context: IosContext) -> TaiResult<IosContext> {
        set_bench_arg(&mut context.options);
        Ok(context)
    }
}

impl Task<AndroidContext> for SetBenchArg {
    fn run(&self, mut context: AndroidContext) -> TaiResult<AndroidContext> {
        set_bench_arg(&mut context.options);
        Ok(context)
    }
}

fn set_bench_arg(options: &mut Options) {
    if let Command::Bench | Command::Benches = options.command {
        let mut args_with_bench = vec!["--bench".to_string()];
        if let Some(ref mut opts) = options.binary {
            if let Some(ref args) = opts.args {
                args_with_bench.extend_from_slice(args);
            }
            opts.args = Some(args_with_bench);
        };
    }
}
