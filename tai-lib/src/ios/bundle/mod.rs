use std::path::PathBuf;

use crate::ios::compiler::BuildUnit;

pub mod bundler;
pub mod signing;

const APP_DISPLAY_NAME: &'static str = "cargo-tai";

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

