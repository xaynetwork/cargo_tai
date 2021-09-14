use cfg_expr::targets::{get_builtin_target_by_triple, TargetInfo};

use crate::{
    common::{
        command::Command,
        compiler::{compile_benches, compile_static_lib, compile_tests, BuildUnit},
        options::CompilerOptions,
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

pub struct BuildBuildUnit;

impl Task<Context> for BuildBuildUnit {
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let general_opt = &context.requested.general;

        let build_units = if let Command::Build = general_opt.command {
            build_universal(&general_opt.compiler)?
        } else {
            let cmd = match general_opt.command {
                Command::Bench => bench_command()?,
                Command::Test => test_command()?,
                Command::Benches => benches_command()?,
                Command::Tests => tests_command()?,
                Command::Build => unreachable!(),
            };
            match general_opt.command {
                Command::Bench | Command::Benches => compile_benches(cmd, &general_opt.compiler)?,
                Command::Test | Command::Tests => compile_tests(cmd, &general_opt.compiler)?,
                Command::Build => unreachable!(),
            }
        };

        context.build_units = Some(build_units);
        Ok(context)
    }
}

fn valid_targets() -> TaiResult<Vec<TargetInfo<'static>>> {
    let targets = vec![
        get_builtin_target_by_triple("aarch64-apple-ios")
            .ok_or_else(|| anyhow::anyhow!("invalid target triple"))?
            .to_owned(),
        get_builtin_target_by_triple("x86_64-apple-ios")
            .ok_or_else(|| anyhow::anyhow!("invalid target triple"))?
            .to_owned(),
    ];
    Ok(targets)
}

fn build_universal(opts: &CompilerOptions) -> TaiResult<Vec<BuildUnit>> {
    valid_targets()?
        .into_iter()
        .map(|target| {
            let mut target_opts = opts.clone();
            target_opts.target = target;
            compile_static_lib(build_lib_command()?, &target_opts)?
                .pop()
                .ok_or_else(|| anyhow::anyhow!("no build unit"))
        })
        .collect()
}
