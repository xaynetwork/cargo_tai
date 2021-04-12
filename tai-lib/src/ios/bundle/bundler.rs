use std::{
    fs::{copy, create_dir, create_dir_all, File},
    path::{Path, PathBuf},
};

use anyhow::{bail, Context};
use cfg_expr::targets::{Arch, TargetInfo};
use serde::Serialize;
use tracing::{debug, instrument};

use crate::{bundle::BuildBundle, compiler::BuildUnit, TaiResult};

const APP_DISPLAY_NAME: &'static str = "cargo-tai";
const INFO_PLIST: &'static str = "Info.plist";

#[instrument(name = "bundle", fields(unit = %unit.name), skip(unit, bundles_root, app_id, resources))]
pub fn create_bundle<P: AsRef<Path>>(
    unit: BuildUnit,
    bundles_root: P,
    resources: &Option<Vec<(String, PathBuf)>>,
    app_id: &str,
) -> TaiResult<BuildBundle> {
    let version_root = bundles_root.as_ref().join(&unit.name);
    let bundle_root = version_root.join(format!("{}.app", APP_DISPLAY_NAME));

    create_dir_all(&bundle_root)
        .with_context(|| format!("Failed to create bundle root {:?}", bundle_root))?;
    debug!("create dir: {:?}", bundle_root);

    let to = bundle_root.join(&unit.name);
    copy(&unit.executable, &to)
        .with_context(|| format!("Failed to copy executable {:?}", unit.executable))?;
    debug!("copy {:?} to {:?}", &unit.executable, to);

    create_plist(&bundle_root, &unit, app_id)
        .with_context(|| format!("Failed to create {}", INFO_PLIST))?;
    copy_resources(&bundle_root, resources)?;

    Ok(BuildBundle {
        root: bundle_root,
        build_unit: unit,
    })
}

fn copy_resources<P: AsRef<Path>>(
    bundle_root: P,
    resources: &Option<Vec<(String, PathBuf)>>,
) -> TaiResult<()> {
    if let Some(resources) = resources {
        debug!("copy resources");
        let test_data_root = bundle_root.as_ref().join(tai_util::DATA_DIR_NAME);
        create_dir(&test_data_root)
            .with_context(|| format!("Failed to create resource root {:?}", test_data_root))?;
        debug!("create dir: {:?}", test_data_root);

        let copied: TaiResult<Vec<()>> = resources
            .iter()
            .map(|(id, local_path)| {
                let remote_path = test_data_root.join(id);
                copy(local_path, &remote_path)
                    .with_context(|| format!("Failed to copy resource {:?}", local_path))?;
                debug!("copy {:?} to {:?}", local_path, remote_path);
                Ok(())
            })
            .collect();
        copied?;
    }

    Ok(())
}

#[derive(Clone, Debug, Serialize)]
pub struct InfoPlist<'a> {
    #[serde(rename = "CFBundleExecutable")]
    pub cf_bundle_executable: &'a str,
    #[serde(rename = "CFBundleIdentifier")]
    pub cf_bundle_identifier: &'a str,
    #[serde(rename = "UIRequiredDeviceCapabilities")]
    pub ui_required_device_capabilities: Vec<&'a str>,
    #[serde(rename = "CFBundleVersion")]
    pub cf_bundle_version: &'a str,
    #[serde(rename = "CFBundleShortVersionString")]
    pub cf_bundle_short_version_string: &'a str,
    #[serde(rename = "UIFileSharingEnabled")]
    pub ui_file_sharing_enabled: bool,
    #[serde(rename = "LSSupportsOpeningDocumentsInPlace")]
    pub ls_supports_opening_documents_in_place: bool,
}

fn create_plist<P: AsRef<Path>>(
    bundle_root: P,
    build_unit: &BuildUnit,
    app_id: &str,
) -> TaiResult<PathBuf> {
    let path = bundle_root.as_ref().join(INFO_PLIST);

    debug!("create file: {:?}", path);
    let plist = File::create(&path)?;
    plist::to_writer_xml(
        plist,
        &InfoPlist {
            cf_bundle_executable: &build_unit.name,
            cf_bundle_identifier: app_id,
            ui_required_device_capabilities: vec![to_apple_arch(&build_unit.target)?],
            cf_bundle_version: "1",
            cf_bundle_short_version_string: "1.0",
            ui_file_sharing_enabled: true,
            ls_supports_opening_documents_in_place: true,
        },
    )?;
    Ok(path)
}

fn to_apple_arch(target: &TargetInfo) -> TaiResult<&'static str> {
    match target.arch {
        Arch::aarch64 => Ok("arm64"),
        Arch::x86_64 => Ok("x86_64"),
        _ => bail!("unknown target"),
    }
}