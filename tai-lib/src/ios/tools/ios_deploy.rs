use std::{
    fmt::{self, Display, Formatter},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::anyhow;
use serde::{Deserialize, Deserializer};

use crate::{common::tools::command_ext::ExitStatusExt, TaiResult};

const IOS_DEPLOY: &str = "ios-deploy";

pub struct IosDeployLaunch<'a, 'e> {
    device: String,
    bundle: PathBuf,
    args: Option<&'a [String]>,
    envs: Option<&'e [(String, String)]>,
    non_interactive: bool,
    debug: bool,
    no_wifi: bool,
    verbose: bool,
}

impl<'a, 'e> IosDeployLaunch<'a, 'e> {
    pub fn new<P: AsRef<Path>>(device: &str, bundle: P) -> Self {
        Self {
            device: device.to_owned(),
            bundle: bundle.as_ref().to_owned(),
            args: None,
            envs: None,
            non_interactive: false,
            debug: false,
            no_wifi: false,
            verbose: false,
        }
    }

    pub fn args(&mut self, args: &'a [String]) -> &mut Self {
        self.args = Some(args);
        self
    }
    pub fn envs(&mut self, envs: &'e [(String, String)]) -> &mut Self {
        self.envs = Some(envs);
        self
    }

    pub fn non_interactive(&mut self) -> &mut Self {
        self.non_interactive = true;
        self
    }

    pub fn debug(&mut self) -> &mut Self {
        self.debug = true;
        self
    }

    pub fn no_wifi(&mut self) -> &mut Self {
        self.no_wifi = true;
        self
    }

    pub fn verbose(&mut self) -> &mut Self {
        self.verbose = true;
        self
    }

    pub fn execute(&mut self) -> TaiResult<()> {
        let mut cmd = Command::new(IOS_DEPLOY);
        if !self.verbose {
            cmd.stdout(Stdio::null());
            cmd.stderr(Stdio::null());
        }

        cmd.arg("--id").arg(&self.device);

        self.non_interactive
            .then(|| ())
            .map(|_| cmd.arg("--noninteractive"));
        self.debug.then(|| ()).map(|_| cmd.arg("--debug"));
        self.no_wifi.then(|| ()).map(|_| cmd.arg("--no-wifi"));

        if let Some(args) = self.args {
            cmd.args(&["--args", &args.join(" ")]);
        };

        if let Some(envs) = self.envs {
            let envs_as_string = envs
                .iter()
                .map(|(key, value)| format!("{}={}", key, value))
                .collect::<Vec<String>>()
                .join(" ");
            cmd.args(&["--envs", &envs_as_string]);
        };

        cmd.arg("--bundle").arg(&self.bundle);

        cmd.status()?.expect_success("failed to run ios_deploy")
    }
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
