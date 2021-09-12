use crate::{
    android::compiler::{bench_command, benches_command, test_command, tests_command},
    command::Command,
    compiler::{compile_benches, compile_tests},
    task::Task,
    TaiResult,
};

use super::Context;

pub struct BuildBuildUnit;

impl Task for BuildBuildUnit {
    type Context = Context;

    fn run(&self, mut context: Self::Context) -> TaiResult<Self::Context> {
        let sdk = context.android_sdk()?;
        let general_opt = &context.requested.general;

        let cmd = match general_opt.command {
            Command::Bench => bench_command(sdk, &context.requested)?,
            Command::Test => test_command(sdk, &context.requested)?,
            Command::Benches => benches_command(sdk, &context.requested)?,
            Command::Tests => tests_command(sdk, &context.requested)?,
            Command::Build => todo!(),
        };

        let build_units = match general_opt.command {
            Command::Bench | Command::Benches => compile_benches(cmd, &general_opt.compiler)?,
            Command::Test | Command::Tests => compile_tests(cmd, &general_opt.compiler)?,
            Command::Build => todo!(),
        };
        context.build_units = Some(build_units);

        Ok(context)
    }
}
