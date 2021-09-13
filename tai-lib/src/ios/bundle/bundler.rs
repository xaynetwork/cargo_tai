use std::{
    fs::{copy, create_dir_all, remove_dir_all, File},
    path::{Path, PathBuf},
};

use anyhow::{bail, Context};
use cfg_expr::targets::{Arch, TargetInfo};
use serde::Serialize;
use tracing::{debug, instrument};

use crate::{
    bundle::{copy_resources, BuildBundle},
    compiler::BuildUnit,
    TaiResult,
};

pub const APP_DISPLAY_NAME: &str = "cargo-tai";
const INFO_PLIST: &str = "Info.plist";

#[instrument(name = "bundle", fields(unit = %unit.name), skip(unit, bundles_root, app_id, resources))]
pub fn create_bundle<P: AsRef<Path>>(
    unit: BuildUnit,
    bundles_root: P,
    resources: &Option<Vec<(String, PathBuf)>>,
    app_id: &str,
) -> TaiResult<BuildBundle> {
    let version_root = bundles_root
        .as_ref()
        .join(unit.target.triple)
        .join(&unit.name);

    if version_root.exists() {
        remove_dir_all(&version_root)
            .with_context(|| format!("Failed to remove old bundle {}", version_root.display()))?;
    }

    let bundle_root = version_root.join(format!("{}.app", APP_DISPLAY_NAME));

    create_dir_all(&bundle_root)
        .with_context(|| format!("Failed to create bundle root {}", bundle_root.display()))?;
    debug!("create dir: {}", bundle_root.display());

    let to = bundle_root.join(&unit.name);
    copy(&unit.artifact, &to)
        .with_context(|| format!("Failed to copy artifact {}", unit.artifact.display()))?;
    debug!("copy {} to {}", &unit.artifact.display(), to.display());

    create_plist(&bundle_root, &unit, app_id)
        .with_context(|| format!("Failed to create {}", INFO_PLIST))?;
    copy_resources(&bundle_root, resources)?;

    Ok(BuildBundle {
        root: bundle_root,
        build_unit: unit,
    })
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

    debug!("create file: {}", path.display());
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
