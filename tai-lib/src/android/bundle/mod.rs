use std::{
    fs::{copy, create_dir_all},
    path::{Path, PathBuf},
};

use tracing::{debug, instrument};

use crate::{
    bundle::{copy_resources, BuildBundle},
    compiler::BuildUnit,
    TaiResult,
};

#[instrument(name = "bundle", fields(unit = %unit.name), skip(unit, bundles_root, resources))]
pub fn create_bundle<P: AsRef<Path>>(
    unit: BuildUnit,
    bundles_root: P,
    resources: &Option<Vec<(String, PathBuf)>>,
) -> TaiResult<BuildBundle> {
    let bundle_root = bundles_root.as_ref().join(&unit.name);

    create_dir_all(&bundle_root)?;
    debug!("create dir: {}", bundle_root.display());
    let to = bundle_root.join(&unit.name);
    copy(&unit.artifact, &to)?;
    debug!("copy {} to {}", &unit.artifact.display(), to.display());
    copy_resources(&bundle_root, resources)?;

    Ok(BuildBundle {
        root: bundle_root,
        build_unit: unit,
    })
}
