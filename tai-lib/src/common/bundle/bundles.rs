use std::{
    fs::{copy, create_dir_all},
    path::{Path, PathBuf},
};

use anyhow::{Context, Error};
use tracing::debug;

use crate::{
    common::{compiler::BuiltUnit, project::ProjectMetadata},
    TaiResult,
};

use super::{BuiltBundle, BuiltBundles};

pub fn create_bundles(
    units: Vec<BuiltUnit>,
    project_metadata: &ProjectMetadata,
    f: impl Fn(BuiltUnit, &PathBuf) -> TaiResult<BuiltBundle>,
) -> TaiResult<BuiltBundles> {
    let tai_target = &project_metadata.tai_target;
    create_dir_all(&tai_target)?;

    let bundles = units
        .into_iter()
        .map(|unit| f(unit, &tai_target))
        .collect::<Result<Vec<_>, Error>>()?;

    Ok(BuiltBundles { bundles })
}

pub fn copy_resources_bundle<P: AsRef<Path>>(
    bundle_root: P,
    resources: &Option<Vec<(String, PathBuf)>>,
) -> TaiResult<()> {
    if let Some(resources) = resources {
        debug!("copy resources");
        let test_data_root = bundle_root.as_ref().join(tai_util::DATA_DIR_NAME);
        create_dir_all(&test_data_root).with_context(|| {
            format!(
                "Failed to create resource root {}",
                test_data_root.display()
            )
        })?;
        debug!("create dir: {}", test_data_root.display());

        copy_resources(test_data_root, resources)?;
    }

    Ok(())
}

pub fn copy_resources<P: AsRef<Path>>(
    dest_dir: P,
    resources: &[(String, PathBuf)],
) -> TaiResult<()> {
    let copied: TaiResult<Vec<()>> = resources
        .iter()
        .map(|(id, local_path)| {
            let remote_path = dest_dir.as_ref().join(id);
            copy(local_path, &remote_path)
                .with_context(|| format!("Failed to copy resource {}", local_path.display()))?;
            debug!("copy {} to {}", local_path.display(), remote_path.display());
            Ok(())
        })
        .collect();
    copied?;
    Ok(())
}
