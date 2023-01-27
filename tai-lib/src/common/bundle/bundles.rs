use std::{
    fs::{copy, create_dir_all},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context, Error};
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

pub fn copy_libraries<P: AsRef<Path>>(
    dest_dir: P,
    libraries: &[PathBuf]
) -> TaiResult<()> {
    debug!("copy libraries");

    let lib_root = dest_dir.as_ref().join(tai_util::LIB_DIR_NAME);
    create_dir_all(&lib_root).with_context(|| {
        format!(
            "Failed to create library root {}",
            lib_root.display()
        )
    })?;
    debug!("create dir: {}", lib_root.display());

    let copied: TaiResult<Vec<()>> = libraries
        .iter()
        .map(|local_path| {
            let file_name =
                local_path
                    .file_name()
                    .ok_or_else(|| anyhow!("Cannot get file name for {}", local_path.display()))?;
            let remote_path = lib_root.join(file_name);
            copy(local_path, &remote_path)
                .with_context(|| format!("Failed to copy library {}", local_path.display()))?;
            debug!("copy {} to {}", local_path.display(), remote_path.display());
            Ok(())
        })
        .collect();
    copied?;

    Ok(())
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
