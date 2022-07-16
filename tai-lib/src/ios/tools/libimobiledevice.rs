use std::{
    fmt::{self, Display, Formatter},
    process::Command,
};

use anyhow::anyhow;
use serde::{Deserialize, Deserializer};

use crate::TaiResult;

const IDEVICE_ID: &str = "idevice_id";
const IDEVICEINFO: &str = "ideviceinfo";

pub fn list_devices() -> TaiResult<Vec<Device>> {
    let output = Command::new(IDEVICE_ID).arg("-l").output()?;
    let udids = String::from_utf8_lossy(&output.stdout);
    udids.lines().map(device_info).collect()
}

pub fn device_info(udid: &str) -> TaiResult<Device> {
    let output = Command::new(IDEVICEINFO)
        .args(&["-s", "-x", "-u", udid])
        .output()?;

    plist::from_bytes(&output.stdout).map_err(|err| {
        anyhow!(format!(
            "Failed to deserialize device info for device id `{}`. Error: {}",
            udid, err
        ))
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct Device {
    #[serde(rename = "UniqueDeviceID")]
    pub id: String,
    #[serde(rename = "DeviceName")]
    pub name: String,
    #[serde(rename = "CPUArchitecture")]
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
            "arm64e" => Self::Aarch64,
            _ => Self::Unsupported(string),
        }
    }
}

impl Display for CpuArch {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}
