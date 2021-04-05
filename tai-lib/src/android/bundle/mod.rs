use std::{
    fs::{copy, create_dir_all},
    path::Path,
};

use tracing::debug;

use crate::{bundle::BuildBundle, compiler::BuildUnit, TaiResult};

pub fn create_bundle<P: AsRef<Path>>(unit: BuildUnit, bundles_root: P) -> TaiResult<BuildBundle> {
    let bundle_root = bundles_root.as_ref().join(&unit.name);

    create_dir_all(&bundle_root)?;
    debug!("create dir: {:?}", bundle_root);
    let to = bundle_root.join(&unit.name);
    copy(&unit.executable, &to)?;
    debug!("copy {:?} to {:?}", &unit.executable, to);

    Ok(BuildBundle {
        root: bundle_root,
        build_unit: unit,
    })
}
