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

pub fn create_bundles<T: AsRef<Path>>(
    units: Vec<BuiltUnit>,
    target: T,
    f: impl Fn(BuiltUnit, &Path) -> TaiResult<BuiltBundle>,
) -> TaiResult<BuiltBundles> {
    let bundles = units
        .into_iter()
        .map(|unit| f(unit, target.as_ref()))
        .collect::<Result<Vec<_>, Error>>()
        .with_context(|| "Failed to built all bundles".to_string())?;
    Ok(BuiltBundles { bundles })
}

#[derive(Debug, Deserialize)]
pub struct Resource {
    pub package_id: String,
    pub resource_source: PathBuf,
    pub resource_destination: PathBuf,
}

pub fn find_resources<R: AsRef<Path>>(
    package_id: &str,
    resources_dir: R,
    package_graph: &PackageGraph,
) -> TaiResult<Vec<Resource>> {
    read_dir(resources_dir)?
        .into_iter()
        .collect::<Result<Vec<DirEntry>, _>>()?
        .into_iter()
        .filter_map(|entry| {
            entry.path().is_dir().then(|| {
                read_dir(entry.path())?
                    .into_iter()
                    .collect::<Result<Vec<DirEntry>, _>>()?
                    .into_iter()
                    .filter_map(|entry| {
                        entry.path().is_file().then(|| {
                            debug!("Found resource metadata at `{}`", entry.path().display());
                            let content = read(entry.path())?;
                            serde_json::from_slice::<Resource>(&content)
                                .map_err::<anyhow::Error, _>(Into::into)
                        })
                    })
                    .collect::<Result<Vec<Resource>, _>>()
                    .map_err::<anyhow::Error, _>(Into::into)
            })
        })
        .collect::<Result<Vec<Vec<Resource>>, _>>()?
        .into_iter()
        .flatten()
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

pub fn copy_resources<D: AsRef<Path>>(dest_dir: D, resources: &[Resource]) -> TaiResult<()> {
    resources
        .iter()
        .map(|res| {
            let res_rel_dest_dir = res
                .resource_destination
                .parent()
                .expect("Failed to determine parent of resource file");
            let bundle_dest_dir = dest_dir.as_ref().join(res_rel_dest_dir);
            create_dir_all(&bundle_dest_dir)?;
            let remote_path = bundle_dest_dir.join(
                res.resource_destination
                    .file_name()
                    .expect("Failed to get filename of resource file"),
            );
            debug!(
                "Copy resource `{}` to `{}`",
                res.resource_source.display(),
                remote_path.display()
            );
            copy(&res.resource_source, &remote_path)?;

            Ok(())
        })
        .collect::<TaiResult<Vec<()>>>()?;
    Ok(())
}
