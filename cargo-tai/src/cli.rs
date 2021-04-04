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
        }
    }
}
