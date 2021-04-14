use std::path::PathBuf;

use anyhow::{anyhow, Error};
use cfg_expr::targets::{get_builtin_target_by_triple, TargetInfo};
use structopt::{clap::ArgSettings, StructOpt};
use tai_lib::task::{self, AndroidOptions, Mode};

#[derive(StructOpt, Debug)]
pub enum Options {
    Bench(GeneralOptions),
    Test(GeneralOptions),
}

#[derive(StructOpt, Debug)]
pub struct GeneralOptions {
    /// Build for the target triples
    #[structopt(long, parse(try_from_str = parse_target), long_help =
    r"Build for the target triples

Supported targets:
- `x86_64-apple-ios` (macOS only)
- `aarch64-apple-ios` (macOS only)
- `x86_64-linux-android`
- `aarch64-linux-android`
- `i686-linux-android`
- `armv7-linux-androideabi`"
    )]
    target: TargetInfo<'static>,

    /// Android platform version: only required when "target" is "*-linux-android*"
    #[structopt(
        long,
        default_value = "21",
        required_ifs(&[
            ("target", "x86_64-linux-android"),
            ("target", "aarch64-linux-android"),
            ("target", "i686-linux-android"),
            ("target", "armv7-linux-androideabi"),
        ])
    )]
    android_platform: u8,

    /// The path to the android ndk: only required when "target" is "*-linux-android*"
    ///
    /// Example:
    ///
    /// `cargo-tai test --android_ndk ~/Library/Android/sdk/ndk/22.1.7171670`
    #[structopt(
        long,
        required_ifs(&[
            ("target", "x86_64-linux-android"),
            ("target", "aarch64-linux-android"),
            ("target", "i686-linux-android"),
            ("target", "armv7-linux-androideabi"),
        ]),
        env = "ANDROID_NDK_HOME"
    )]
    android_ndk: PathBuf,

    /// A comma-separated list of arguments to pass to the app when launching it.
    ///
    /// Example:
    ///
    /// `cargo-tai test --args -Z,unstable-options,--report-time`
    #[structopt(short, long, allow_hyphen_values = true, use_delimiter = true)]
    args: Option<Vec<String>>,

    /// Environment variables to pass to the app when launching it. Format: `key=value`
    ///
    /// Example:
    ///
    /// `cargo-tai test --envs TAI_1=1 TAI_2=2`
    #[structopt(short, long, parse(try_from_str = parse_key_val))]
    envs: Option<Vec<(String, String)>>,

    /// Ressources to include in the app. Format: `id=local_path`
    ///
    /// Example:
    ///
    /// `cargo-tai test -r test_txt=./data/text.txt`
    #[structopt(short, long, parse(try_from_str = parse_key_val))]
    resources: Option<Vec<(String, PathBuf)>>,

    /// Arguments that are passed to `cargo`. See `cargo build --help`.
    ///
    /// Example:
    ///
    /// `cargo-tai test -- --release`
    #[structopt(set = ArgSettings::Last)]
    cargo_args: Vec<String>,
}

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn std::error::Error>>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + 'static,
    U: std::str::FromStr,
    U::Err: std::error::Error + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

fn parse_target(src: &str) -> Result<TargetInfo<'static>, Error> {
    let target = get_builtin_target_by_triple(src).ok_or(anyhow!("unsupported target"))?;
    Ok(target.to_owned())
}

impl From<Options> for task::Options {
    fn from(opt: Options) -> Self {
        let (mode, general_opts) = match opt {
            Options::Bench(opts) => (Mode::Bench, opts),
            Options::Test(opts) => (Mode::Test, opts),
        };

        Self {
            mode,
            target: general_opts.target,
            args: general_opts.args,
            envs: general_opts.envs,
            resources: general_opts.resources,
            android: AndroidOptions {
                platform: general_opts.android_platform,
                ndk: general_opts.android_ndk,
            },
            cargo_args: general_opts.cargo_args,
        }
    }
}
