use tracing::instrument;

use crate::{
    common::{
        command::Command,
        compiler::{compile_benches, compile_staticlib, compile_tests},
        task::Task,
    },
    ios::compiler::{
        bench_command,
        benches_command,
        build_lib_command,
        test_command,
        tests_command,
    },
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
            Command::Build => build_lib_command()?,
        };
        let built_units = match opts.command {
            Command::Bench | Command::Benches => compile_benches(cmd, &opts.compiler)?,
            Command::Test | Command::Tests => compile_tests(cmd, &opts.compiler)?,
            Command::Build => compile_staticlib(cmd, &opts.compiler)?,
        };

        context.built_units = Some(built_units);
        Ok(context)
    }
}

// fn valid_targets() -> TaiResult<Vec<TargetInfo<'static>>> {
//     let targets = vec![
//         get_builtin_target_by_triple("aarch64-apple-ios")
//             .ok_or_else(|| anyhow::anyhow!("invalid target triple"))?
//             .to_owned(),
//         get_builtin_target_by_triple("x86_64-apple-ios")
//             .ok_or_else(|| anyhow::anyhow!("invalid target triple"))?
//             .to_owned(),
//     ];
//     Ok(targets)
// }

// fn build_universal(opts: &CompilerOptions) -> TaiResult<Vec<BuildUnit>> {
//     valid_targets()?
//         .into_iter()
//         .map(|target| {
//             let mut target_opts = opts.clone();
//             target_opts.target = target;
//             compile_static_lib(build_lib_command()?, &target_opts)?
//                 .pop()
//                 .ok_or_else(|| anyhow::anyhow!("no build unit"))
//         })
//         .collect()
// }
