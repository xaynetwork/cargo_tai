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
use tracing::{debug, info};

use crate::{
    common::bundle::BuiltBundle,
    ios::tools::{codesign::CodeSign, security},
    TaiResult,
};

const ENTITLEMENTS_XCENT: &str = "entitlements.xcent";

#[derive(Debug)]
pub struct SigningSettings {
    pub identity_name: String,
    pub app_id: String,
    pub entitlements: String,
    pub mobile_provision_path: PathBuf,
    pub mobile_provision: MobileProvision,
}

#[derive(Debug, Deserialize)]
pub struct MobileProvision {
    #[serde(rename = "ProvisionedDevices")]
    pub provisioned_devices: Vec<String>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ExpirationDate")]
    pub expiration_date: plist::Date,
    #[serde(rename = "DeveloperCertificates")]
    pub developer_certificates: Vec<Data>,
}

#[derive(Debug, Deserialize)]
pub struct Data(#[serde(with = "serde_bytes")] Vec<u8>);

pub fn sign_bundle<E: AsRef<Path>>(
    bundle: &BuiltBundle,
    settings: &SigningSettings,
    entitlements: E,
) -> TaiResult<()> {
    info!("Sign bundle: `{}`", bundle.build_unit.name);

    debug!(
        "Sign `{}` using identity `{}` and profile `{}`",
        bundle.root.display(),
        settings.identity_name,
        settings.mobile_provision_path.display()
    );

    CodeSign::new(&settings.identity_name, &[&bundle.root])
        .entitlements(entitlements)
        .execute()
}

pub fn create_entitlements_file<D: AsRef<Path>>(dest: D, entitlements: &str) -> TaiResult<PathBuf> {
    let path = dest.as_ref().join(ENTITLEMENTS_XCENT);
    debug!("Create entitlements file: `{}`", path.display());

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

pub fn find_signing_settings<P: AsRef<Path>>(profile: P) -> TaiResult<SigningSettings> {
    let output = security::decode_cms(profile.as_ref())?.stdout;
    let mobile_provision: MobileProvision = plist::from_bytes(&output).with_context(|| {
        format!(
            "Failed to load provisioning profile: {}",
            profile.as_ref().display()
        )
    })?;

    let expiration_date: SystemTime = mobile_provision.expiration_date.into();
    if expiration_date < SystemTime::now() {
        bail!(
            "Provisioning profile expired on: {}",
            <DateTime<Utc>>::from(expiration_date)
        );
    }

    let cert_decoded = &mobile_provision
        .developer_certificates
        .first()
        .ok_or_else(|| anyhow!("Missing team identifier"))?
        .0;
    let cert_encoded = base64::encode(cert_decoded);

    let mut with_header = String::from("-----BEGIN CERTIFICATE-----\n");
    with_header.push_str(&cert_encoded);
    with_header.push_str("\n-----END CERTIFICATE-----");

    let identity_name = get_signing_identity_name(with_header.as_bytes())?;

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
        .ok_or_else(|| anyhow!("Missing app Id"))?
        .to_string();

    Ok(SigningSettings {
        identity_name,
        app_id,
        entitlements,
        mobile_provision_path: profile.as_ref().to_path_buf(),
        mobile_provision,
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
