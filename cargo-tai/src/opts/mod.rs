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

use self::{
    android::AndroidOptions,
    binary::BinaryOptions,
    cli::CliOptions,
    compiler::CompilerOptions,
    ios::IosOptions,
};

#[derive(Debug, Parser)]
pub enum Options {
    #[clap(about = "Benchmark only the specified bench target")]
    Bench(LocalRun),
    #[clap(about = "Test only the specified test target")]
    Test(LocalRun),
    #[clap(about = "Benchmark all benches")]
    Benches(LocalRun),
    #[clap(about = "Test all tests")]
    Tests(LocalRun),
}

#[derive(Debug, Parser)]
pub struct LocalRun {
    #[clap(flatten)]
    cli: CliOptions,

    #[clap(flatten)]
    compiler: CompilerOptions,

    #[clap(flatten)]
    binary: BinaryOptions,

    #[clap(flatten)]
    android: AndroidOptions,

    #[clap(flatten)]
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use cfg_expr::targets::get_builtin_target_by_triple;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_tests_with_resource() {
        let o = Options::parse_from("cargo-tai tests --target x86_64-apple-ios".split_whitespace());
        let o = match o {
            Options::Tests(o) => o,
            _ => panic!(""),
        };

        assert_eq!(
            &o.compiler.target,
            get_builtin_target_by_triple("x86_64-apple-ios").unwrap()
        );
    }

    #[test]
    fn test_tests_with_additional_binary_arguments() {
        let o = Options::parse_from(
            "cargo-tai tests --target x86_64-apple-ios --args -Z,unstable-options,--report-time"
                .split_whitespace(),
        );
        let o = match o {
            Options::Tests(o) => o,
            _ => panic!(""),
        };

        assert_eq!(
            &o.compiler.target,
            get_builtin_target_by_triple("x86_64-apple-ios").unwrap()
        );
        assert_eq!(
            &o.binary.args.unwrap(),
            &vec![
                "-Z".to_string(),
                "unstable-options".to_string(),
                "--report-time".to_string()
            ]
        );
    }

    #[test]
    fn test_tests_with_cargo_arguments() {
        let o = Options::parse_from(
            "cargo-tai tests --target x86_64-apple-ios --args test_x86_64_ios -- --release"
                .split_whitespace(),
        );
        let o = match o {
            Options::Tests(o) => o,
            _ => panic!(""),
        };

        assert_eq!(
            &o.compiler.target,
            get_builtin_target_by_triple("x86_64-apple-ios").unwrap()
        );
        assert_eq!(
            &o.binary.args.unwrap(),
            &vec!["test_x86_64_ios".to_string()]
        );
        assert_eq!(&o.compiler.cargo_args, &vec!["--release".to_string(),]);
    }

    #[test]
    fn test_test_with_cargo_arguments() {
        let o = Options::parse_from(
            "cargo-tai test --target x86_64-apple-ios --args test_x86_64_ios -- integration"
                .split_whitespace(),
        );
        let o = match o {
            Options::Test(o) => o,
            _ => panic!(""),
        };

        assert_eq!(
            &o.compiler.target,
            get_builtin_target_by_triple("x86_64-apple-ios").unwrap()
        );
        assert_eq!(
            &o.binary.args.unwrap(),
            &vec!["test_x86_64_ios".to_string()]
        );
        assert_eq!(&o.compiler.cargo_args, &vec!["integration".to_string(),]);
    }

    #[test]
    fn test_test_with_cargo_ndk_arguments_and_cargo_arguments() {
        let o = Options::parse_from(
            "cargo-tai test --target x86_64-linux-android --android-api-lvl 21 --android-ndk path --cargo-ndk-args --no-strip,--bindgen --args test_x86_64_android -- integration"
                .split_whitespace(),
        );
        let o = match o {
            Options::Test(o) => o,
            _ => panic!(""),
        };

        assert_eq!(
            &o.compiler.target,
            get_builtin_target_by_triple("x86_64-linux-android").unwrap()
        );
        assert_eq!(o.android.api_lvl.unwrap(), 21);
        assert_eq!(o.android.ndk.unwrap(), PathBuf::from("path"));
        assert!(o.android.sdk.is_none());
        assert_eq!(
            &o.android.cargo_ndk_args.unwrap(),
            &vec!["--no-strip".to_string(), "--bindgen".to_string()]
        );
        assert_eq!(
            &o.binary.args.unwrap(),
            &vec!["test_x86_64_android".to_string()]
        );
        assert_eq!(&o.compiler.cargo_args, &vec!["integration".to_string(),]);
    }
}
