use std::{
    fs::{copy, create_dir, remove_dir_all},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, Context, Error};
use tracing::debug;

use crate::{compiler::BuildUnit, TaiResult};

use super::{BuildBundle, BuildBundles, BUNDLES_ROOT_NAME};

pub fn create_bundles(
    units: Vec<BuildUnit>,
    f: impl Fn(BuildUnit, &PathBuf) -> TaiResult<BuildBundle>,
) -> TaiResult<BuildBundles> {
    let unit = units.get(0).ok_or_else(|| anyhow!("no units to bundle"))?;
    let root = unit
        .executable
        .parent()
        .map(|p| p.parent())
        .flatten()
        .ok_or_else(|| anyhow!("cannot find bundle root"))?
        .to_path_buf();

    let bundles_root = root.join(BUNDLES_ROOT_NAME);

    match (bundles_root.is_dir(), bundles_root.is_file()) {
        (false, false) => create_dir(&bundles_root)?,
        (true, false) => {
            remove_dir_all(&bundles_root)?;
            create_dir(&bundles_root)?;
        }
        (false, true) => bail!("bundle root is a file"),
        _ => unreachable!(),
    };

    let bundles = units
        .into_iter()
        .map(|unit| f(unit, &bundles_root))
        .collect::<Result<Vec<_>, Error>>()?;

    Ok(BuildBundles {
        root: bundles_root,
        bundles,
    })
}

pub fn copy_resources<P: AsRef<Path>>(
    bundle_root: P,
    resources: &Option<Vec<(String, PathBuf)>>,
) -> TaiResult<()> {
    if let Some(resources) = resources {
        debug!("copy resources");
        let test_data_root = bundle_root.as_ref().join(tai_util::DATA_DIR_NAME);
        create_dir(&test_data_root).with_context(|| {
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
