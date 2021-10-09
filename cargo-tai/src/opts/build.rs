use std::path::PathBuf;

use structopt::StructOpt;
use tai_lib::common::opts;

#[derive(StructOpt, Debug)]
pub struct BuildOptions {
    // Template dir.
    #[structopt(short, long)]
    pub template_dir: PathBuf,

    // Out dir where the test artifacts are copied to.
    #[structopt(short, long)]
    pub out_dir: PathBuf,
}

impl From<BuildOptions> for opts::BuildOptions {
    fn from(options: BuildOptions) -> Self {
        opts::BuildOptions {
            template_dir: options.template_dir,
            out_dir: options.out_dir,
        }
    }
}
