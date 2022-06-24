use tracing::instrument;

use crate::{
    common::{
        command::Command,
        compiler::{compile_benches, compile_tests},
        task::Task,
    },
    ios::compiler::{bench_command, benches_command, test_command, tests_command},
    TaiResult,
};

use super::Context;

pub struct BuildBuiltUnits;

impl Task<Context> for BuildBuiltUnits {
    #[instrument(name = "build_built_units", skip(self, context))]
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let opts = &context.opts;

        let cmd = match opts.command {
            Command::Bench => bench_command()?,
            Command::Test => test_command()?,
            Command::Benches => benches_command()?,
            Command::Tests => tests_command()?,
        };
        let built_units = match opts.command {
            Command::Bench | Command::Benches => compile_benches(cmd, &opts.compiler)?,
            Command::Test | Command::Tests => compile_tests(cmd, &opts.compiler)?,
        };

        context.built_units = Some(built_units);
        Ok(context)
    }
}
