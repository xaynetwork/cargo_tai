use std::{
    fs::{create_dir, remove_dir_all},
    path::PathBuf,
};

use anyhow::{anyhow, bail, Error};

use crate::{compiler::BuildUnit, TaiResult};

use super::{BuildBundle, BuildBundles, BUNDLES_ROOT_NAME};

pub fn create_bundles(
    units: Vec<BuildUnit>,
    f: impl Fn(BuildUnit, &PathBuf) -> TaiResult<BuildBundle>,
) -> TaiResult<BuildBundles> {
    let unit = units.get(0).ok_or(anyhow!("no units to bundle"))?;
    let root = unit
        .executable
        .parent()
        .ok_or(anyhow!("no units to bundle"))?
        .parent()
        .ok_or(anyhow!("no units to bundle"))?
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
