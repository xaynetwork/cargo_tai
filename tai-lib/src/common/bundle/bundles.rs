use std::{
    fs::{copy, create_dir_all, read, read_dir, DirEntry},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context, Error};
use guppy::{graph::PackageGraph, PackageId};
use serde::Deserialize;
use tracing::debug;

use crate::{common::compiler::BuiltUnit, TaiResult};

use super::{BuiltBundle, BuiltBundles};

pub fn create_bundles(
    units: Vec<BuiltUnit>,
    target: &Path,
    f: impl Fn(BuiltUnit, &Path) -> TaiResult<BuiltBundle>,
) -> TaiResult<BuiltBundles> {
    let bundles = units
        .into_iter()
        .map(|unit| f(unit, target))
        .collect::<Result<Vec<_>, Error>>()
        .with_context(|| "Failed to built all bundles".to_string())?;
    Ok(BuiltBundles { bundles })
}

pub fn copy_resources<P: AsRef<Path>>(
    dest_dir: P,
    resources: &[(String, PathBuf)],
) -> TaiResult<()> {
    debug!("copy resources");

    let test_data_root = dest_dir.as_ref().join(tai_util::DATA_DIR_NAME);
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

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct Resource {
    pub package_id: String,
    pub resource_path_absolute: String,
    pub resource_path_relative: String,
}

pub fn find_resources(
    package_id: &str,
    resources_dir: &PathBuf,
    package_graph: &PackageGraph,
) -> TaiResult<Vec<Resource>> {
    read_dir(resources_dir)?
        .into_iter()
        .collect::<Result<Vec<DirEntry>, _>>()?
        .into_iter()
        .filter_map(|entry| {
            entry.path().is_file().then(|| {
                let content = read(entry.path())?;
                serde_json::from_slice::<Resource>(&content).map_err(|err| anyhow!(err))
            })
        })
        .collect::<Result<Vec<Resource>, _>>()?
        .into_iter()
        .filter_map(|dep| {
            package_graph
                .depends_on(
                    &PackageId::new(package_id.to_owned()),
                    &PackageId::new(dep.package_id.to_owned()),
                )
                .map_or_else(|err| Some(Err(anyhow!(err))), |_| Some(Ok(dep)))
        })
        .collect()
}

pub fn copy_resources2<P: AsRef<Path>>(dest_dir: P, resources: &[Resource]) -> TaiResult<()> {
    resources
        .iter()
        .map(|res| {
            let rel_path = PathBuf::from(&res.resource_path_relative);
            let rel_dirs = rel_path.parent().unwrap();
            let dest_resource_dir = dest_dir.as_ref().join(rel_dirs);
            create_dir_all(&dest_resource_dir)?;
            let remote_path = dest_resource_dir.join(rel_path.file_name().unwrap());
            copy(&res.resource_path_absolute, &remote_path)?;

            Ok(())
        })
        .collect::<TaiResult<Vec<()>>>()?;
    Ok(())
}
