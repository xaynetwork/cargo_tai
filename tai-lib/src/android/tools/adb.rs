use std::{
    io::ErrorKind,
    path::Path,
    process::{Command, Output},
};

use anyhow::anyhow;
use cfg_expr::targets::Arch;
use once_cell::sync::OnceCell;

use crate::{common::tools::command_ext::ExitStatusExt, TaiResult};

use super::AndroidEnv;

static DEVICE_REGEX: OnceCell<regex::Regex> = OnceCell::new();

#[derive(Debug)]
pub struct Device {
    pub id: String,
    pub arch: Arch<'static>,
}

pub fn devices(env: &AndroidEnv) -> TaiResult<Vec<Device>> {
    let output = Command::new(&env.adb).arg("devices").output().map_err(|error| {
        match error.kind() {
            ErrorKind::NotFound => anyhow!("Cannot list devices because adb is not installed at {}", env.adb.to_str().unwrap()),
            _ => anyhow!("adb devices: {}", error)
        }
    })?;
    let device_regex =
        DEVICE_REGEX.get_or_init(|| regex::Regex::new(r#"^(\S+)\tdevice\r?$"#).unwrap());

    String::from_utf8(output.stdout)?
        .split('\n')
        .skip(1)
        .into_iter()
        .try_fold(vec![], |mut devices, line| {
            if let Some(caps) = device_regex.captures(line) {
                let id = caps[1].to_owned();
                let arch = arch(env, &id)?;

                devices.push(Device { id, arch })
            }
            Ok(devices)
        })
}

pub fn arch(env: &AndroidEnv, device: &str) -> TaiResult<Arch<'static>> {
    let output = Command::new(&env.adb)
        .args(["-s", device, "shell", "getprop", "ro.product.cpu.abi"])
        .output()?;
    let cpu_arch: CpuArch = String::from_utf8(output.stdout)?.trim().into();
    Ok(cpu_arch.into())
}

pub fn mkdir<P: AsRef<Path>>(env: &AndroidEnv, device: &str, path: P) -> TaiResult<()> {
    Command::new(&env.adb)
        .args(["-s", device, "shell", "mkdir", "-p"])
        .arg(path.as_ref())
        .status()?
        .expect_success("failed to create directory")
}

pub fn sync<FP: AsRef<Path>, TP: AsRef<Path>>(
    env: &AndroidEnv,
    device: &str,
    from: FP,
    to: TP,
) -> TaiResult<()> {
    Command::new(&env.adb)
        .args(["-s", device, "push", "--sync"])
        .args([from.as_ref(), to.as_ref()])
        .status()?
        .expect_success("failed to sync files")
}

pub fn rm<P: AsRef<Path>>(env: &AndroidEnv, device: &str, path: P) -> TaiResult<()> {
    Command::new(&env.adb)
        .args(["-s", device, "shell", "rm", "-rf"])
        .arg(path.as_ref())
        .status()?
        .expect_success("failed to remove files/directories")
}

pub fn chmod<P: AsRef<Path>>(env: &AndroidEnv, device: &str, path: P) -> TaiResult<()> {
    Command::new(&env.adb)
        .args(["-s", device, "shell", "chmod", "755"])
        .arg(path.as_ref())
        .status()?
        .expect_success("failed to chmod file/directory")
}

pub fn run(env: &AndroidEnv, device: &str, start_script: &str) -> TaiResult<Output> {
    Command::new(&env.adb)
        .args(["-s", device, "shell"])
        .arg(start_script)
        .output()
        .map_err(|err| anyhow!("{}", err))
}

// #TODO replace with https://github.com/rust-windowing/android-ndk-rs/blob/master/ndk-build/src/target.rs

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

impl From<CpuArch> for &str {
    fn from(arch: CpuArch) -> Self {
        match arch {
            CpuArch::Aarch64 => "arm64-v8a",
            CpuArch::ARMv7 => "armeabi-v7a",
            CpuArch::X86 => "x86",
            CpuArch::X86_64 => "x86_64",
            CpuArch::Unsupported(_) => panic!(""),
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

impl From<Arch<'_>> for CpuArch {
    fn from(arch: Arch) -> Self {
        match arch {
            Arch::aarch64 => CpuArch::Aarch64,
            Arch::arm => CpuArch::ARMv7,
            Arch::x86 => CpuArch::X86,
            Arch::x86_64 => CpuArch::X86_64,
            _ => panic!(""),
        }
    }
}
