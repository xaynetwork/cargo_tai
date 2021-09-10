use crate::{
    command::Command,
    compiler::{compile_benches, compile_tests},
    ios::compiler::{bench_command, benches_command, test_command, tests_command},
    options::GeneralOptions,
};

pub mod physical;
pub mod simulator;

const APP_ID: &str = "cargo-tai";

fn compile_build_units(
    general_opt: &GeneralOptions,
) -> Result<Vec<crate::compiler::BuildUnit>, anyhow::Error> {
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
    Ok(build_units)
}
