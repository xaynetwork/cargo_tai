use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    time::SystemTime,
};

use anyhow::{anyhow, bail};
use once_cell::sync::OnceCell;
use openssl::{nid::Nid, x509::X509};
use plist;
use serde::Deserialize;
use tracing::{debug, instrument};

use crate::{
    ios::{
        bundle::BuildBundle,
        platform::physical::APP_NAME,
        tools::{codesign, security},
    },
    TaiResult,
};

const ENTITLEMENTS_XCENT: &'static str = "entitlements.xcent";
const PROFILE_DIR: &'static str = "Library/MobileDevice/Provisioning Profiles";
static IDENTITY_REGEX: OnceCell<regex::Regex> = OnceCell::new();

#[derive(Debug)]
pub struct SigningSettings {
    pub identity: SigningIdentity,
    pub app_id: String,
    pub entitlements: String,
    pub provision: PathBuf,
}

#[derive(Debug, Clone)]
pub struct SigningIdentity {
    pub id: String,
    pub name: String,
    pub team: String,
}

#[derive(Deserialize, Debug)]
struct MobileProvision {
    #[serde(rename = "ProvisionedDevices")]
    provisioned_devices: Vec<String>,
    #[serde(rename = "TeamIdentifier")]
    team_identifier: Vec<String>,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "ExpirationDate")]
    expiration_date: plist::Date,
}

#[instrument(name = "sign", skip(bundle, settings))]
pub fn sign_bundle(
    bundle: &BuildBundle,
    settings: &SigningSettings,
    entitlements: &PathBuf,
) -> TaiResult<()> {
    debug!(
        "will sign {:?} with team: {} using identity: {} and profile: {:?}",
        bundle.root, settings.identity.team, settings.identity.name, settings.provision
    );

    codesign::sign(&settings.identity.name, &entitlements, &bundle.root)
}

#[instrument(name = "find_signing_identities", skip(device_id))]
pub fn find_signing_settings(device_id: &str) -> TaiResult<SigningSettings> {
    let identities = find_identities()?;
    debug!("found {:?}", identities);

    let profiles = dirs::home_dir()
        .ok_or(anyhow!("cannot find home directory"))?
        .join(PROFILE_DIR);

    let mut setting = None;
    for file in fs::read_dir(profiles)? {
        if let Ok(entry) = file {
            if entry
                .path()
                .extension()
                .map_or_else(|| false, |ex| ex == "mobileprovision")
                == true
            {
                debug!("considering profile {:?}", entry.path());
                match read_and_validate_profile(&identities, device_id, entry.path()) {
                    Ok((identity, entitlements, app_id)) => {
                        setting = SigningSettings {
                            identity: identity.clone(),
                            entitlements,
                            app_id,
                            provision: entry.path(),
                        }
                        .into();
                        break;
                    }
                    Err(err) => debug!("profile unsuitable: {}", err),
                }
            }
        }
    }

    setting.ok_or(anyhow!("no signing profiles available"))
}

pub fn create_entitlements_file(bundles_root: &PathBuf, entitlements: &str) -> TaiResult<PathBuf> {
    let path = bundles_root.join(ENTITLEMENTS_XCENT);
    debug!("create entitlements file: {:?}", path);

    let mut plist = File::create(&path)?;
    writeln!(plist, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
    writeln!(
        plist,
        r#"<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">"#
    )?;
    writeln!(plist, r#"<plist version="1.0"><dict>"#)?;
    writeln!(plist, "{}", entitlements)?;
    writeln!(plist, r#"</dict></plist>"#)?;

    Ok(path)
}

fn get_subject_from_x509(cert_name: &str) -> TaiResult<String> {
    let cert = security::find_certificate(cert_name)?.stdout;
    let x509 = X509::from_pem(&cert)?;
    let subject = x509
        .subject_name()
        .entries_by_nid(Nid::ORGANIZATIONALUNITNAME)
        .next()
        .unwrap()
        .data()
        .as_utf8()?;
    Ok(subject.to_string())
}

fn find_identities() -> TaiResult<Vec<SigningIdentity>> {
    let identity_regex = IDENTITY_REGEX
        .get_or_init(|| regex::Regex::new(r#"^ *[0-9]+\) ([A-Z0-9]{40}) "(.+)"$"#).unwrap());

    let output = security::find_identities()?.stdout;
    let lines = String::from_utf8(output)?;
    lines.split('\n').try_fold(vec![], |mut identities, line| {
        if let Some(caps) = identity_regex.captures(line) {
            let name: String = caps[2].into();
            if name.starts_with("iPhone Developer: ") || name.starts_with("Apple Development:") {
                let subject = get_subject_from_x509(&name)?;
                identities.push(SigningIdentity {
                    id: caps[1].into(),
                    name: caps[2].into(),
                    team: subject.to_string(),
                });
            }
        }
        Ok(identities)
    })
}

fn read_and_validate_profile<'a, P: AsRef<Path>>(
    identities: &'a [SigningIdentity],
    device_id: &str,
    profile: P,
) -> TaiResult<(&'a SigningIdentity, String, String)> {
    let output = security::decode_cms(profile.as_ref())?.stdout;
    let mobile_provision: MobileProvision = plist::from_bytes(&output)?;

    let expiration_date: SystemTime = mobile_provision.expiration_date.into();
    if expiration_date < SystemTime::now() {
        bail!("profile expired on: {:?}", expiration_date);
    }

    if !mobile_provision
        .provisioned_devices
        .iter()
        .any(|d| d == device_id)
    {
        bail!("device: {} not in profile", device_id);
    }

    if !mobile_provision.name.ends_with(APP_NAME) {
        bail!("app in profile does not match ({})", mobile_provision.name);
    }

    let team = mobile_provision
        .team_identifier
        .first()
        .ok_or_else(|| anyhow!("missing team identifier"))?;

    let identity = identities
        .iter()
        .find(|i| &i.team == team)
        .ok_or_else(|| anyhow!("no identity for team"))?;

    let entitlements = String::from_utf8(output)?
        .split('\n')
        .skip_while(|line| !line.contains("<key>Entitlements</key>"))
        .skip(2)
        .take_while(|line| !line.contains("</dict>"))
        .collect::<Vec<&str>>()
        .join("\n");

    let app_id = mobile_provision
        .name
        .split(' ')
        .last()
        .ok_or_else(|| anyhow!("missing app id"))?
        .to_string();

    Ok((identity, entitlements, app_id))
}
