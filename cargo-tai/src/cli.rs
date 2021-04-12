use anyhow::{anyhow, Error};
use cfg_expr::targets::{get_builtin_target_by_triple, TargetInfo};
use structopt::{clap::ArgSettings, StructOpt};
use tai_lib::task::{self, Mode};

#[derive(StructOpt, Debug)]
pub enum Options {
    Bench(GeneralOptions),
    Test(GeneralOptions),
}

#[derive(StructOpt, Debug)]
pub struct GeneralOptions {
    /// Build for the target triples
    #[structopt(long, parse(try_from_str = parse_target))]
    pub target: TargetInfo<'static>,

    /// Android platform version: only required when `target` is set to `*-linux-android*`
    #[structopt(
        name = "android platform version",
        default_value = "21",
        required_ifs(&[
            ("target", "x86_64-linux-android"),
            ("target", "aarch64-linux-android"),
            ("target", "i686-linux-android"),
            ("target", "armv7-linux-androideabi"),
        ])
    )]
    android_platform: u8,

    /// Environment variables to pass to the app when launching it. Format: key=value
    #[structopt(long, parse(try_from_str = parse_key_val))]
    envs: Option<Vec<(String, String)>>,

    /// Arguments to pass to the app when launching it.
    #[structopt(long)]
    args: Option<Vec<String>>,

    /// Arguments that are passed to cargo. See `cargo build --help`.
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
            android_platform: general_opts.android_platform,
            envs: general_opts.envs,
            args: general_opts.args,
            cargo_args: general_opts.cargo_args,
        }
    }
}
