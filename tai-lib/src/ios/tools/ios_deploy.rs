use std::{
    fmt::{self, Display, Formatter},
    path::Path,
    process::Command,
};

use serde::{Deserialize, Deserializer};

use crate::{command_ext::ExitStatusExt, TaiResult};

const IOS_DEPLOY: &'static str = "ios-deploy";

pub fn launch_app<P: AsRef<Path>>(bundle_root: P, args: &[&str], envs: &[&str]) -> TaiResult<()> {
    Command::new(IOS_DEPLOY)
        .args(&[
            "--noninteractive",
            "--debug",
            "--args",
            &args.join(" "),
            "--envs",
            &envs.join(" "),
            "--bundle",
        ])
        .arg(bundle_root.as_ref())
        .status()?
        .expect_success("failed to launch app")
}

pub fn list_device() -> TaiResult<Option<Device>> {
    let output = Command::new(IOS_DEPLOY)
        .args(&["--detect", "--detect", "--timeout", "1", "--json"])
        .output()?;
    let devices: Option<Devices> = serde_json::from_slice(&output.stdout)?;
    if let Some(devices) = devices {
        Ok(Some(devices.device))
    } else {
        Ok(None)
    }
}

#[derive(Deserialize, Debug)]
struct Devices {
    #[serde(rename = "Event")]
    event: String,
    #[serde(rename = "Device")]
    device: Device,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Device {
    #[serde(rename = "DeviceIdentifier")]
    pub id: String,
    #[serde(rename = "DeviceName")]
    pub name: String,
    #[serde(rename = "modelName")]
    pub model: String,
    #[serde(rename = "modelArch")]
    #[serde(deserialize_with = "deserialize_cpu_arch")]
    pub arch: CpuArch,
    #[serde(rename = "ProductVersion")]
    pub version: String,
}

#[derive(Debug, Clone)]
pub enum CpuArch {
    Aarch64,
    Unsupported(String),
}

impl CpuArch {
    pub fn as_str(&self) -> &str {
        match self {
            CpuArch::Aarch64 => "aarch64",
            CpuArch::Unsupported(inner) => inner,
        }
    }
}

fn deserialize_cpu_arch<'de, D>(deserializer: D) -> Result<CpuArch, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;
    Ok(CpuArch::from(buf))
}

impl From<String> for CpuArch {
    fn from(string: String) -> Self {
        match string.as_str() {
            "arm64" => Self::Aarch64,
            _ => Self::Unsupported(string),
        }
    }
}

impl Display for CpuArch {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}
