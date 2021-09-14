use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    time::SystemTime,
};

use anyhow::{anyhow, bail, Context};
use chrono::{DateTime, Utc};
use openssl::{nid::Nid, x509::X509};
use serde::Deserialize;
use tracing::{debug, instrument};

use crate::{
    common::bundle::BuildBundle,
    ios::tools::{codesign, security},
    TaiResult,
};

const ENTITLEMENTS_XCENT: &str = "entitlements.xcent";

#[derive(Debug)]
pub struct SigningSettings {
    pub identity_name: String,
    pub app_id: String,
    pub entitlements: String,
    pub provision: PathBuf,
}

#[derive(Deserialize, Debug)]
struct MobileProvision {
    #[serde(rename = "ProvisionedDevices")]
    provisioned_devices: Vec<String>,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "ExpirationDate")]
    expiration_date: plist::Date,
    #[serde(rename = "DeveloperCertificates")]
    developer_certificates: Vec<Data>,
}

#[derive(Deserialize, Debug)]
struct Data(#[serde(with = "serde_bytes")] Vec<u8>);

#[instrument(name = "sign", skip(bundle, settings))]
pub fn sign_bundle(
    bundle: &BuildBundle,
    settings: &SigningSettings,
    entitlements: &Path,
) -> TaiResult<()> {
    debug!(
        "will sign {} using identity: {} and profile: {}",
        bundle.root.display(),
        settings.identity_name,
        settings.provision.display()
    );

    codesign::sign(&settings.identity_name, &entitlements, &bundle.root)
}

#[instrument(name = "entitlements", skip(dest, entitlements))]
pub fn create_entitlements_file(dest: &Path, entitlements: &str) -> TaiResult<PathBuf> {
    let path = dest.join(ENTITLEMENTS_XCENT);
    debug!("create entitlements file: {}", path.display());

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

pub fn find_signing_settings<P: AsRef<Path>>(
    device_id: &str,
    profile: P,
) -> TaiResult<SigningSettings> {
    let output = security::decode_cms(profile.as_ref())?.stdout;
    let mobile_provision: MobileProvision = plist::from_bytes(&output).with_context(|| {
        format!(
            "Failed to load provisioning profile: {}",
            profile.as_ref().display()
        )
    })?;

    let cert_decoded = &mobile_provision
        .developer_certificates
        .first()
        .ok_or_else(|| anyhow!("missing team identifier"))?
        .0;
    let cert_encoded = base64::encode(cert_decoded);

    let mut with_header = String::from("-----BEGIN CERTIFICATE-----\n");
    with_header.push_str(&cert_encoded);
    with_header.push_str("\n-----END CERTIFICATE-----");

    let identity_name = get_signing_identity_name(with_header.as_bytes())?;

    let expiration_date: SystemTime = mobile_provision.expiration_date.into();
    if expiration_date < SystemTime::now() {
        bail!(
            "provisioning profile expired on: {}",
            <DateTime<Utc>>::from(expiration_date)
        );
    }

    if !mobile_provision
        .provisioned_devices
        .iter()
        .any(|d| d == device_id)
    {
        bail!("device: {} not in provisioning profile", device_id);
    }

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

    Ok(SigningSettings {
        identity_name,
        app_id,
        entitlements,
        provision: profile.as_ref().to_path_buf(),
    })
}

fn get_signing_identity_name(cert: &[u8]) -> TaiResult<String> {
    let x509 = X509::from_pem(cert)?;
    let subject = x509
        .subject_name()
        .entries_by_nid(Nid::COMMONNAME)
        .next()
        .unwrap()
        .data()
        .as_utf8()?;
    Ok(subject.to_string())
}
