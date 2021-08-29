use crate::{
    compiler::{compile_benches, compile_tests},
    ios::compiler::{bench_command, benches_command, test_command, tests_command},
    task::{GeneralOptions, Task},
};

pub mod physical;
pub mod simulator;

const APP_ID: &str = "cargo-tai";

fn compile_build_units(
    general_opt: &GeneralOptions,
) -> Result<Vec<crate::compiler::BuildUnit>, anyhow::Error> {
    let cmd = match general_opt.task {
        Task::Bench => bench_command()?,
        Task::Test => test_command()?,
        Task::Benches => benches_command()?,
        Task::Tests => tests_command()?,
    };
    let build_units = match general_opt.task {
        Task::Bench | Task::Benches => compile_benches(cmd, &general_opt.compiler)?,
        Task::Test | Task::Tests => compile_tests(cmd, &general_opt.compiler)?,
    };
    Ok(build_units)
}
