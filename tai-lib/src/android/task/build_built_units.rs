use crate::{
    android::compiler::{bench_command, benches_command, test_command, tests_command},
    common::{
        command::Command,
        compiler::{compile_benches, compile_tests},
        task::Task,
    },
    TaiResult,
};

use super::Context;

pub struct BuildBuiltUnits;

impl Task<Context> for BuildBuiltUnits {
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let sdk = context.android_sdk()?;
        let opts = &context.opts;

        let cmd = match opts.command {
            Command::Bench => bench_command(sdk, &context.opts)?,
            Command::Test => test_command(sdk, &context.opts)?,
            Command::Benches => benches_command(sdk, &context.opts)?,
            Command::Tests => tests_command(sdk, &context.opts)?,
        };

        let built_units = match opts.command {
            Command::Bench | Command::Benches => compile_benches(cmd, &opts.compiler)?,
            Command::Test | Command::Tests => compile_tests(cmd, &opts.compiler)?,
        };
        context.built_units = Some(built_units);

        Ok(context)
    }
}
