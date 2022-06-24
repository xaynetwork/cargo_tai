use crate::{
    android::{
        compiler::{bench_command, benches_command, test_command, tests_command},
        tools::AndroidSdk,
    },
    common::{
        command::Command,
        compiler::{compile_benches, compile_tests, BuiltUnit},
        opts::Options,
        task::Task,
    },
    TaiResult,
};

use super::Context;

pub struct BuiltUnits(pub Vec<BuiltUnit>);

pub struct BuildBuiltUnits;

impl Task<Context> for BuildBuiltUnits {
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let sdk: &AndroidSdk = context.get();
        let opts: &Options = context.get();

        let cmd = match opts.command {
            Command::Bench => bench_command(sdk, opts)?,
            Command::Test => test_command(sdk, opts)?,
            Command::Benches => benches_command(sdk, opts)?,
            Command::Tests => tests_command(sdk, opts)?,
        };

        let built_units = match opts.command {
            Command::Bench | Command::Benches => compile_benches(cmd, &opts.compiler)?,
            Command::Test | Command::Tests => compile_tests(cmd, &opts.compiler)?,
        };
        context.insert(BuiltUnits(built_units));

        Ok(context)
    }
}
