use std::path::PathBuf;

use crate::compiler::BuildUnit;

mod bundles;

pub use bundles::{copy_resources, create_bundles};

#[derive(Debug)]
pub struct BuildBundles {
    // pub root: PathBuf,
    pub bundles: Vec<BuildBundle>,
}

#[derive(Debug)]
pub struct BuildBundle {
    pub root: PathBuf,
    pub build_unit: BuildUnit,
}
