use std::{
    fs::{copy, create_dir_all, remove_dir_all},
    path::{Path, PathBuf},
};

use guppy::graph::PackageGraph;
use tracing::{debug, info, instrument};

use crate::{
    common::{
        bundle::{copy_resources, find_resources, BuiltBundle},
        compiler::BuiltUnit,
    },
    TaiResult,
};

#[instrument(level = "debug", name = "bundle", fields(unit = %unit.name), skip(unit, bundles_root, resources_dir, package_graph))]
pub fn create_bundle<P: AsRef<Path>>(
    unit: BuiltUnit,
    bundles_root: P,
    resources_dir: &PathBuf,
    package_graph: &PackageGraph,
) -> TaiResult<BuiltBundle> {
    info!("Create Android app bundle for `{}`", unit.name);
    let bundle_root = bundles_root
        .as_ref()
        .join(unit.target.triple)
        .join(&unit.name);

    if bundle_root.exists() {
        remove_dir_all(&bundle_root)?;
    }

    create_dir_all(&bundle_root)?;
    debug!("Create dir: `{}`", bundle_root.display());
    let to = bundle_root.join(&unit.name);
    copy(&unit.artifact, &to)?;
    debug!("Copy `{}` to `{}`", &unit.artifact.display(), to.display());

    let resources = find_resources(&unit.package_id, resources_dir, package_graph)?;
    copy_resources(&bundle_root, &resources)?;

    Ok(BuiltBundle {
        root: bundle_root,
        build_unit: unit,
    })
}
