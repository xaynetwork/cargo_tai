use std::path::PathBuf;

use crate::compiler::BuildUnit;

mod bundles;

pub use bundles::create_bundles;

pub const BUNDLES_ROOT_NAME: &'static str = "tai-test";

#[derive(Debug)]
pub struct BuildBundles {
    pub root: PathBuf,
    pub bundles: Vec<BuildBundle>,
}

#[derive(Debug)]
pub struct BuildBundle {
    pub root: PathBuf,
    pub build_unit: BuildUnit,
}
