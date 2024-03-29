use crate::{
    common::{command::Command, opts::Options, task::Task},
    TaiResult,
};

use super::context::Context;

pub struct SetBenchArg;

impl Task<Context> for SetBenchArg {
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        set_bench_arg(context.get_mut());
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
