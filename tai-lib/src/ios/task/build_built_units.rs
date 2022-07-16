use tracing::{debug, instrument};

use crate::{
    common::{
        command::Command,
        compiler::{compile_benches, compile_tests, BuiltUnit},
        opts::Options,
        project::ProjectMetadata,
        task::Task,
    },
    ios::compiler::{bench_command, benches_command, test_command, tests_command},
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
        let opts: &Options = context.get();
        debug!("{:#?}", opts.compiler);

        let cmd = match opts.command {
            Command::Bench => bench_command(&opts.compiler)?,
            Command::Test => test_command(&opts.compiler)?,
            Command::Benches => benches_command(&opts.compiler)?,
            Command::Tests => tests_command(&opts.compiler)?,
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
