use std::{
    fs::{copy, create_dir_all, remove_dir_all},
    path::{Path, PathBuf},
};

use tracing::{debug, instrument};

use crate::{
    common::{
        bundle::{copy_libraries, copy_resources, BuiltBundle},
        compiler::BuiltUnit,
    },
    TaiResult,
};

#[instrument(name = "bundle", fields(unit = %unit.name), skip(unit, bundles_root, resources))]
pub fn create_bundle<P: AsRef<Path>>(
    unit: BuiltUnit,
    bundles_root: P,
    libraries: &Option<Vec<PathBuf>>,
    resources: &Option<Vec<(String, PathBuf)>>,
) -> TaiResult<BuiltBundle> {
    let bundle_root = bundles_root
        .as_ref()
        .join(unit.target.triple)
        .join(&unit.name);

    if bundle_root.exists() {
        remove_dir_all(&bundle_root)?;
    }

    create_dir_all(&bundle_root)?;
    debug!("create dir: {}", bundle_root.display());
    let to = bundle_root.join(&unit.name);
    copy(&unit.artifact, &to)?;
    debug!("copy {} to {}", &unit.artifact.display(), to.display());

    if let Some(libraries) = libraries {
        copy_libraries(&bundle_root, libraries)?;
    }

    if let Some(resources) = resources {
        copy_resources(&bundle_root, resources)?;
    }

    Ok(BuiltBundle {
        root: bundle_root,
        build_unit: unit,
    })
}
