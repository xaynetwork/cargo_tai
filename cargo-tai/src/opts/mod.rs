use clap::Parser;
use tai_lib::common::{
    command::Command,
    opts::{self},
};

pub mod android;
pub mod binary;
pub mod cli;
pub mod compiler;
pub mod ios;
pub mod resource;

use self::{
    android::AndroidOptions,
    binary::BinaryOptions,
    cli::CliOptions,
    compiler::CompilerOptions,
    ios::IosOptions,
    resource::ResourceOptions,
};

#[derive(Parser, Debug)]
pub enum Options {
    #[structopt(about = "Benchmark only the specified bench target")]
    Bench(LocalRun),
    #[structopt(about = "Test only the specified test target")]
    Test(LocalRun),
    #[structopt(about = "Benchmark all benches")]
    Benches(LocalRun),
    #[structopt(about = "Test all tests")]
    Tests(LocalRun),
}

#[derive(Parser, Debug)]
pub struct LocalRun {
    #[structopt(flatten)]
    cli: CliOptions,

    #[structopt(flatten)]
    compiler: CompilerOptions,

    #[structopt(flatten)]
    resources: ResourceOptions,

    #[structopt(flatten)]
    binary: BinaryOptions,

    #[structopt(flatten)]
    android: AndroidOptions,

    #[structopt(flatten)]
    ios: IosOptions,
}

impl From<Options> for opts::Options {
    fn from(opt: Options) -> Self {
        match opt {
            Options::Bench(opts) => from_local_run(Command::Bench, opts),
            Options::Test(opts) => from_local_run(Command::Test, opts),
            Options::Benches(opts) => from_local_run(Command::Benches, opts),
            Options::Tests(opts) => from_local_run(Command::Tests, opts),
        }
    }
}

fn from_local_run(command: Command, options: LocalRun) -> opts::Options {
    opts::Options {
        command,
        compiler: options.compiler.into(),
        resources: options.resources.resources,
        binary: options.binary.into(),
        android: options.android.into(),
        ios: options.ios.into(),
        cli: options.cli.into(),
    }
}

/// Parse a single key-value pair
fn parse_key_val<T, U>(
    s: &str,
) -> Result<(T, U), Box<dyn std::error::Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: std::error::Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}
