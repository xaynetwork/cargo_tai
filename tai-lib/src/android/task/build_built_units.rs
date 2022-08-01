use tracing::instrument;

use crate::{
    android::{
        compiler::{bench_command, benches_command, test_command, tests_command},
        tools::AndroidEnv,
    },
    common::{
        command::Command,
        compiler::{compile_benches, compile_tests, BuiltUnit},
        opts::Options,
        project::ProjectMetadata,
        task::Task,
    },
    TaiResult,
};

use super::Context;

#[derive(Debug)]
pub struct BuiltUnits(pub Vec<BuiltUnit>);

#[derive(Debug)]
pub struct BuildBuiltUnits;

impl Task<Context> for BuildBuiltUnits {
    #[instrument(name = "Build Units", skip_all)]
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let env: &AndroidEnv = context.get();
        let opts: &Options = context.get();

        let cmd = match opts.command {
            Command::Bench => bench_command(env, opts)?,
            Command::Test => test_command(env, opts)?,
            Command::Benches => benches_command(env, opts)?,
            Command::Tests => tests_command(env, opts)?,
        };

        let meta: &ProjectMetadata = context.get();

        let built_units = match opts.command {
            Command::Bench | Command::Benches => compile_benches(cmd, &opts.compiler, meta)?,
            Command::Test | Command::Tests => compile_tests(cmd, &opts.compiler, meta)?,
        };
        context.insert(BuiltUnits(built_units));

        Ok(context)
    }
}
