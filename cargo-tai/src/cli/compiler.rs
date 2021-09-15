use anyhow::{anyhow, Error};
use cfg_expr::targets::{get_builtin_target_by_triple, TargetInfo};
use structopt::{clap::ArgSettings, StructOpt};
use tai_lib::common::options;

#[derive(StructOpt, Debug)]
pub struct CompilerOptions {
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
    pub target: TargetInfo<'static>,

    /// Arguments that are passed to `cargo`. See `cargo build --help`.
    ///
    /// Example:
    ///
    /// `cargo-tai test -- --release`
    #[structopt(set = ArgSettings::Last)]
    pub cargo_args: Vec<String>,
}

fn parse_target(src: &str) -> Result<TargetInfo<'static>, Error> {
    let target = get_builtin_target_by_triple(src).ok_or_else(|| anyhow!("unsupported target"))?;
    Ok(target.to_owned())
}

impl From<CompilerOptions> for options::CompilerOptions {
    fn from(options: CompilerOptions) -> Self {
        options::CompilerOptions {
            target: options.target,
            cargo_args: options.cargo_args,
        }
    }
}
