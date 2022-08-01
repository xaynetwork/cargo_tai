use clap::Parser;
use tai_lib::common::{command, opts};

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
pub enum Command {
    #[clap(about = "Benchmark only the specified bench target")]
    Bench(Options),
    #[clap(about = "Test only the specified test target")]
    Test(Options),
    #[clap(about = "Benchmark all benches")]
    Benches(Options),
    #[clap(about = "Test all tests")]
    Tests(Options),
}

#[derive(Debug, Parser)]
pub struct Options {
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

impl From<Command> for opts::Options {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::Bench(opts) => into_options(command::Command::Bench, opts),
            Command::Test(opts) => into_options(command::Command::Test, opts),
            Command::Benches(opts) => into_options(command::Command::Benches, opts),
            Command::Tests(opts) => into_options(command::Command::Tests, opts),
        }
    }
}

fn into_options(command: command::Command, opts: Options) -> opts::Options {
    opts::Options {
        command,
        compiler: opts.compiler.into(),
        binary: opts.binary.into(),
        android: opts.android.into(),
        ios: opts.ios.into(),
        cli: opts.cli.into(),
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

    use super::*;

    #[test]
    fn test_tests_with_resource() {
        let cmd =
            Command::parse_from("cargo-tai tests --target x86_64-apple-ios".split_whitespace());
        let cmd = match cmd {
            Command::Tests(o) => o,
            _ => panic!(""),
        };

        assert_eq!(
            &cmd.compiler.target,
            get_builtin_target_by_triple("x86_64-apple-ios").unwrap()
        );
    }

    #[test]
    fn test_tests_with_additional_binary_arguments() {
        let cmd = Command::parse_from(
            "cargo-tai tests --target x86_64-apple-ios --args -Z,unstable-options,--report-time"
                .split_whitespace(),
        );
        let cmd = match cmd {
            Command::Tests(o) => o,
            _ => panic!(""),
        };

        assert_eq!(
            &cmd.compiler.target,
            get_builtin_target_by_triple("x86_64-apple-ios").unwrap()
        );
        assert_eq!(
            &cmd.binary.args.unwrap(),
            &vec![
                "-Z".to_string(),
                "unstable-options".to_string(),
                "--report-time".to_string()
            ]
        );
    }

    #[test]
    fn test_tests_with_cargo_arguments() {
        let cmd = Command::parse_from(
            "cargo-tai tests --target x86_64-apple-ios --args test_x86_64_ios -- --release"
                .split_whitespace(),
        );
        let cmd = match cmd {
            Command::Tests(o) => o,
            _ => panic!(""),
        };

        assert_eq!(
            &cmd.compiler.target,
            get_builtin_target_by_triple("x86_64-apple-ios").unwrap()
        );
        assert_eq!(
            &cmd.binary.args.unwrap(),
            &vec!["test_x86_64_ios".to_string()]
        );
        assert_eq!(&cmd.compiler.cargo_args, &vec!["--release".to_string(),]);
    }

    #[test]
    fn test_test_with_cargo_arguments() {
        let cmd = Command::parse_from(
            "cargo-tai test --target x86_64-apple-ios --args test_x86_64_ios -- integration"
                .split_whitespace(),
        );
        let cmd = match cmd {
            Command::Test(o) => o,
            _ => panic!(""),
        };

        assert_eq!(
            &cmd.compiler.target,
            get_builtin_target_by_triple("x86_64-apple-ios").unwrap()
        );
        assert_eq!(
            &cmd.binary.args.unwrap(),
            &vec!["test_x86_64_ios".to_string()]
        );
        assert_eq!(&cmd.compiler.cargo_args, &vec!["integration".to_string(),]);
    }

    #[test]
    fn test_test_with_cargo_ndk_arguments_and_cargo_arguments() {
        let cmd = Command::parse_from(
            "cargo-tai test --target x86_64-linux-android --android-api-lvl 21 --android-ndk path --cargo-ndk-args --no-strip,--bindgen --args test_x86_64_android -- integration"
                .split_whitespace(),
        );
        let cmd = match cmd {
            Command::Test(o) => o,
            _ => panic!(""),
        };

        assert_eq!(
            &cmd.compiler.target,
            get_builtin_target_by_triple("x86_64-linux-android").unwrap()
        );
        assert_eq!(cmd.android.api_lvl.unwrap(), 21);
        assert_eq!(cmd.android.ndk.unwrap(), PathBuf::from("path"));
        assert!(cmd.android.sdk.is_none());
        assert_eq!(
            &cmd.android.cargo_ndk_args.unwrap(),
            &vec!["--no-strip".to_string(), "--bindgen".to_string()]
        );
        assert_eq!(
            &cmd.binary.args.unwrap(),
            &vec!["test_x86_64_android".to_string()]
        );
        assert_eq!(&cmd.compiler.cargo_args, &vec!["integration".to_string(),]);
    }
}
