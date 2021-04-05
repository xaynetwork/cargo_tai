use anyhow::{anyhow, Error};
use cfg_expr::targets::{get_builtin_target_by_triple, TargetInfo};
use structopt::StructOpt;
use tai_lib::task::{self, Mode};

#[derive(StructOpt)]
pub enum Options {
    Bench(GeneralOptions),
    Test(GeneralOptions),
}

#[derive(StructOpt)]
pub struct GeneralOptions {
    /// Build artifacts in release mode, with optimizations
    #[structopt(long)]
    pub release: bool,

    /// Activate all available features
    #[structopt(long = "all-features")]
    pub all_features: bool,

    /// Do not activate the `default` feature
    #[structopt(long = "no-default-features")]
    pub no_default_features: bool,

    /// Space-separated list of features to activate
    #[structopt(value_delimiter = ",")]
    #[structopt(long, default_value = "")]
    pub features: Vec<String>,

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

    /// environment variable that should be passed to the app when launching it key=value
    #[structopt(long, parse(try_from_str = parse_key_val))]
    envs: Option<Vec<(String, String)>>,
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
            release: general_opts.release,
            target: general_opts.target,
            all_features: general_opts.all_features,
            no_default_features: general_opts.no_default_features,
            features: general_opts.features,
            mode,
            android_platform: general_opts.android_platform,
            envs: general_opts.envs,
        }
    }
}
