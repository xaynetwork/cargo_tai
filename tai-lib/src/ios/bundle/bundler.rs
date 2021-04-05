use std::{
    fs::{copy, create_dir_all, File},
    path::{Path, PathBuf},
};

use anyhow::bail;
use cfg_expr::targets::{Arch, TargetInfo};
use serde::Serialize;
use tracing::{debug, instrument};

use crate::{bundle::BuildBundle, compiler::BuildUnit, TaiResult};

const APP_DISPLAY_NAME: &'static str = "cargo-tai";
const INFO_PLIST: &'static str = "Info.plist";

#[instrument(name = "bundle", fields(unit = %unit.name), skip(unit, bundles_root, app_id))]
pub fn create_bundle<P: AsRef<Path>>(
    unit: BuildUnit,
    bundles_root: P,
    app_id: &str,
) -> TaiResult<BuildBundle> {
    let version_root = bundles_root.as_ref().join(&unit.name);
    let bundle_root = version_root.join(format!("{}.app", APP_DISPLAY_NAME));

    create_dir_all(&bundle_root)?;
    debug!("create dir: {:?}", bundle_root);
    let to = bundle_root.join(&unit.name);
    copy(&unit.executable, &to)?;
    debug!("copy {:?} to {:?}", &unit.executable, to);
    create_plist(&bundle_root, &unit, app_id)?;

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
