use crate::{
    command::Command,
    compiler::{compile_benches, compile_tests},
    ios::compiler::{bench_command, benches_command, test_command, tests_command},
    task::Task,
    TaiResult,
};

use super::Context;

pub struct BuildBuildUnit;

impl Task for BuildBuildUnit {
    type Context = Context;

    fn run(&self, mut context: Self::Context) -> TaiResult<Self::Context> {
        let general_opt = &context.requested.general;

        let cmd = match general_opt.command {
            Command::Bench => bench_command()?,
            Command::Test => test_command()?,
            Command::Benches => benches_command()?,
            Command::Tests => tests_command()?,
        };
        let build_units = match general_opt.command {
            Command::Bench | Command::Benches => compile_benches(cmd, &general_opt.compiler)?,
            Command::Test | Command::Tests => compile_tests(cmd, &general_opt.compiler)?,
        };
        context.build_units = Some(build_units);

        Ok(context)
    }
}
