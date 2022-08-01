use clap::Parser;
use std::path::PathBuf;
use tai_lib::common::opts::{self};

#[derive(Debug, Parser)]
pub struct IosOptions {
    #[clap(
        long = "ios-mobile-provision",
        required_if("target", "aarch64-apple-ios")
    )]
    pub mobile_provision: Option<PathBuf>,
}

impl From<IosOptions> for Option<opts::IosOptions> {
    fn from(IosOptions { mobile_provision }: IosOptions) -> Self {
        mobile_provision.map(|mobile_provision| opts::IosOptions { mobile_provision })
    }
}
