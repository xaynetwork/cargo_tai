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

pub struct BuildBuildUnit;

impl Task<Context> for BuildBuildUnit {
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let sdk = context.android_sdk()?;
        let options = &context.options;

        let cmd = match options.command {
            Command::Bench => bench_command(sdk, &context.options)?,
            Command::Test => test_command(sdk, &context.options)?,
            Command::Benches => benches_command(sdk, &context.options)?,
            Command::Tests => tests_command(sdk, &context.options)?,
            Command::Build => todo!(),
        };

        let build_units = match options.command {
            Command::Bench | Command::Benches => compile_benches(cmd, &options.compiler)?,
            Command::Test | Command::Tests => compile_tests(cmd, &options.compiler)?,
            Command::Build => todo!(),
        };
        context.build_units = Some(build_units);

        Ok(context)
    }
}
