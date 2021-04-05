use std::{
    path::Path,
    process::{Command, Output},
};

use anyhow::anyhow;
use cfg_expr::targets::Arch;
use once_cell::sync::OnceCell;

use crate::{command_ext::ExitStatusExt, TaiResult};

const ADB: &'static str = "adb";
static DEVICE_REGEX: OnceCell<regex::Regex> = OnceCell::new();

pub struct Device {
    pub id: String,
    pub arch: Arch<'static>,
}

pub fn devices() -> TaiResult<Vec<Device>> {
    let output = Command::new(ADB).arg("devices").output()?;
    let device_regex =
        DEVICE_REGEX.get_or_init(|| regex::Regex::new(r#"^(\S+)\tdevice\r?$"#).unwrap());

    String::from_utf8(output.stdout)?
        .split('\n')
        .skip(1)
        .into_iter()
        .try_fold(vec![], |mut devices, line| {
            if let Some(caps) = device_regex.captures(line) {
                let id = caps[1].to_owned();
                let arch = arch(&id)?;

                devices.push(Device { id: id, arch })
            }
            Ok(devices)
        })
}

pub fn arch(device: &str) -> TaiResult<Arch<'static>> {
    let output = Command::new(ADB)
        .args(&["-s", device, "shell", "getprop", "ro.product.cpu.abi"])
        .output()?;
    let cpu_arch: CpuArch = String::from_utf8(output.stdout)?.trim().into();
    Ok(cpu_arch.into())
}

pub fn mkdir<P: AsRef<Path>>(device: &str, path: P) -> TaiResult<()> {
    Command::new(ADB)
        .args(&["-s", device, "shell", "mkdir", "-p"])
        .arg(path.as_ref())
        .status()?
        .expect_success("failed to create directory")
}

pub fn sync<FP: AsRef<Path>, TP: AsRef<Path>>(device: &str, from: FP, to: TP) -> TaiResult<()> {
    Command::new(ADB)
        .args(&["-s", device, "push", "--sync"])
        .args(&[from.as_ref(), to.as_ref()])
        .status()?
        .expect_success("failed to sync files")
}

pub fn rm<P: AsRef<Path>>(device: &str, path: P) -> TaiResult<()> {
    Command::new(ADB)
        .args(&["-s", device, "shell", "rm", "-rf"])
        .arg(path.as_ref())
        .status()?
        .expect_success("failed to remove files/directories")
}

pub fn chmod<P: AsRef<Path>>(device: &str, path: P) -> TaiResult<()> {
    Command::new(ADB)
        .args(&["-s", device, "shell", "chmod", "755"])
        .arg(path.as_ref())
        .status()?
        .expect_success("failed to chmod file/directory")
}

pub fn run(device: &str, start_script: &str) -> TaiResult<Output> {
    Command::new(ADB)
        .args(&["-s", device, "shell"])
        .arg(start_script)
        .output()
        .map_err(|err| anyhow!("{}", err))
}

#[derive(Debug, Clone)]
pub enum CpuArch {
    Aarch64,
    ARMv7,
    X86,
    X86_64,
    Unsupported(String),
}

impl From<&str> for CpuArch {
    fn from(string: &str) -> Self {
        match string {
            "arm64-v8a" => Self::Aarch64,
            "armeabi-v7a" => Self::ARMv7,
            "x86" => Self::X86,
            "x86_64" => Self::X86_64,
            _ => Self::Unsupported(String::from(string)),
        }
    }
}

impl From<CpuArch> for Arch<'_> {
    fn from(arch: CpuArch) -> Self {
        match arch {
            CpuArch::Aarch64 => Arch::aarch64,
            CpuArch::ARMv7 => Arch::arm,
            CpuArch::X86 => Arch::x86,
            CpuArch::X86_64 => Arch::x86_64,
            CpuArch::Unsupported(_) => panic!(""),
        }
    }
}
