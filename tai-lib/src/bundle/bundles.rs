use std::{
    fs::{copy, create_dir_all},
    path::{Path, PathBuf},
};

use anyhow::{Context, Error};
use tracing::debug;

use crate::{compiler::BuildUnit, project::ProjectMetadata, TaiResult};

use super::{BuildBundle, BuildBundles};

pub fn create_bundles(
    units: Vec<BuildUnit>,
    project_metadata: &ProjectMetadata,
    f: impl Fn(BuildUnit, &PathBuf) -> TaiResult<BuildBundle>,
) -> TaiResult<BuildBundles> {
    let tai_target_dir = project_metadata.tai_target_dir();
    create_dir_all(&tai_target_dir)?;

    let bundles = units
        .into_iter()
        .map(|unit| f(unit, &tai_target_dir))
        .collect::<Result<Vec<_>, Error>>()?;

    Ok(BuildBundles { bundles })
}

pub fn copy_resources<P: AsRef<Path>>(
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

        let copied: TaiResult<Vec<()>> = resources
            .iter()
            .map(|(id, local_path)| {
                let remote_path = test_data_root.join(id);
                copy(local_path, &remote_path)
                    .with_context(|| format!("Failed to copy resource {}", local_path.display()))?;
                debug!("copy {} to {}", local_path.display(), remote_path.display());
                Ok(())
            })
            .collect();
        copied?;
    }

    Ok(())
}
