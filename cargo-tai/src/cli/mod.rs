use structopt::StructOpt;
use tai_lib::common::{
    command::Command,
    options::{self},
};

use self::{
    android::AndroidOptions,
    binary::BinaryOptions,
    build::BuildOptions,
    compiler::CompilerOptions,
    ios::IosOptions,
};

pub mod android;
pub mod binary;
pub mod build;
pub mod compiler;
pub mod ios;

#[derive(StructOpt, Debug)]
pub enum Options {
    #[structopt(about = "Benchmark only the specified bench target")]
    Bench(LocalRun),
    #[structopt(about = "Test only the specified test target")]
    Test(LocalRun),
    #[structopt(about = "Benchmark all benches")]
    Benches(LocalRun),
    #[structopt(about = "Test all tests")]
    Tests(LocalRun),
    #[structopt(about = "...")]
    Build(NativeTestBuild),
}

#[derive(StructOpt, Debug)]
pub struct LocalRun {
    #[structopt(flatten)]
    compiler: CompilerOptions,

    #[structopt(flatten)]
    binary: BinaryOptions,

    #[structopt(flatten)]
    android: AndroidOptions,

    #[structopt(flatten)]
    ios: IosOptions,
}

#[derive(StructOpt, Debug)]
pub struct NativeTestBuild {
    #[structopt(flatten)]
    compiler: CompilerOptions,

    #[structopt(flatten)]
    build: BuildOptions,

    #[structopt(flatten)]
    android: AndroidOptions,

    #[structopt(flatten)]
    ios: IosOptions,
}

impl From<Options> for options::Options {
    fn from(opt: Options) -> Self {
        match opt {
            Options::Bench(opts) => from_local_run(Command::Bench, opts),
            Options::Test(opts) => from_local_run(Command::Test, opts),
            Options::Benches(opts) => from_local_run(Command::Benches, opts),
            Options::Tests(opts) => from_local_run(Command::Tests, opts),
            Options::Build(opts) => from_native_test_build(Command::Build, opts),
        }
    }
}

fn from_local_run(command: Command, options: LocalRun) -> options::Options {
    options::Options {
        command,
        compiler: options.compiler.into(),
        binary: options.binary.into(),
        build: None,
        android: options.android.into(),
        ios: options.ios.into(),
    }
}

fn from_native_test_build(command: Command, options: NativeTestBuild) -> options::Options {
    options::Options {
        command,
        compiler: options.compiler.into(),
        binary: None,
        build: Some(options.build.into()),
        android: options.android.into(),
        ios: options.ios.into(),
    }
}
