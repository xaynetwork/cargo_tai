use std::{
    fs::{copy, create_dir, create_dir_all, remove_dir_all, File},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, Error};
use cfg_expr::targets::{Arch, TargetInfo};
use serde::Serialize;
use tracing::{debug, instrument};

use crate::{compiler::BuildUnit, TaiResult};

use super::{BuildBundle, BuildBundles};

const BUNDLES_ROOT_NAME: &'static str = "tai-test";

fn create_bundle<P: AsRef<Path>>(unit: BuildUnit, bundles_root: P) -> TaiResult<BuildBundle> {
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

pub fn create_bundles(units: Vec<BuildUnit>) -> TaiResult<BuildBundles> {
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
        .map(|unit| create_bundle(unit, &bundles_root))
        .collect::<Result<Vec<_>, Error>>()?;

    Ok(BuildBundles {
        root: bundles_root,
        bundles,
    })
}
