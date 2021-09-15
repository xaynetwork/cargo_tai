use std::path::PathBuf;
use structopt::StructOpt;
use tai_lib::common::options::{self};

#[derive(StructOpt, Debug)]
pub struct IosOptions {
    #[structopt(
        long = "ios-mobile-provision",
        required_if("target", "aarch64-apple-ios")
    )]
    pub mobile_provision: Option<PathBuf>,
}

impl From<IosOptions> for Option<options::IosOptions> {
    fn from(IosOptions { mobile_provision }: IosOptions) -> Self {
        mobile_provision.map(|mobile_provision| options::IosOptions { mobile_provision })
    }
}
