use std::path::PathBuf;

use crate::common::compiler::BuiltUnit;

mod bundles;

pub use bundles::{copy_libraries, copy_resources, create_bundles};

#[derive(Debug)]
pub struct BuiltBundles {
    // pub root: PathBuf,
    pub bundles: Vec<BuiltBundle>,
}

#[derive(Debug)]
pub struct BuiltBundle {
    pub root: PathBuf,
    pub build_unit: BuiltUnit,
}
