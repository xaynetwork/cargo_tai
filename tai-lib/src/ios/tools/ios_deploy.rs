use std::{
    fmt::{self, Display, Formatter},
    path::Path,
    process::Command,
};

use anyhow::anyhow;
use serde::{Deserialize, Deserializer};

use crate::{command_ext::ExitStatusExt, TaiResult};

const IOS_DEPLOY: &str = "ios-deploy";

pub fn launch_app<P: AsRef<Path>>(
    bundle_root: P,
    args: &Option<Vec<String>>,
    envs: &Option<Vec<(String, String)>>,
) -> TaiResult<()> {
    let mut cmd = Command::new(IOS_DEPLOY);
    cmd.args(&["--noninteractive", "--debug", "--no-wifi"]);

    if let Some(args) = args {
        cmd.args(&["--args", &args.join(" ")]);
    };

    if let Some(envs) = envs {
        let envs_as_string = envs
            .iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<String>>()
            .join(" ");
        cmd.args(&["--envs", &envs_as_string]);
    };

    cmd.arg("--bundle")
        .arg(bundle_root.as_ref())
        .status()?
        .expect_success(&format!("{} command failed", IOS_DEPLOY))
}

pub fn list_device() -> TaiResult<Vec<Device>> {
    let output = Command::new(IOS_DEPLOY)
        .args(&["--detect", "--timeout", "1", "--json", "--no-wifi"])
        .output()?;
    // ios-deploy does not emit a valid json array, therefore we need to manipulate the output first
    let output = format!(
        "[ {} ]",
        String::from_utf8_lossy(&output.stdout).replace("}{", "},{")
    );

    let devices: Vec<Devices> = serde_json::from_str(&output)
        .map_err(|_| anyhow!("Failed to deserialize ios_deploy output"))?;

    Ok(devices.into_iter().map(|device| device.device).collect())
}

#[derive(Deserialize, Debug)]
pub struct Devices {
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
