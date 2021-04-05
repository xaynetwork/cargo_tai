use std::{path::PathBuf, process::Command};

use cfg_expr::targets::TargetInfo;
use tracing::debug;

use crate::{task::Options, TaiResult};

use super::tools::AndroidSdk;

#[cfg(target_os = "macos")]
const HOST_ARCH: &str = "darwin-x86_64";
#[cfg(target_os = "linux")]
const HOST_ARCH: &str = "linux-x86_64";

// ~/Library/Android/sdk/ndk/<22.1.7171670>/toolchains/llvm/prebuilt/<darwin-x86_64>/<x86_64-linux-android>/bin/

fn clang_suffix(
    target: &TargetInfo<'static>,
    host_arch: &str,
    android_platform: u8,
    postfix: &str,
) -> PathBuf {
    let tool_triple = to_clang_suffix(target);
    [
        "toolchains",
        "llvm",
        "prebuilt",
        host_arch,
        "bin",
        &format!("{}{}-clang{}", tool_triple, android_platform, postfix),
    ]
    .iter()
    .collect()
}

pub fn to_clang_suffix(target: &TargetInfo<'static>) -> &'static str {
    match target.triple {
        "arm-linux-androideabi" => "armv7a-linux-androideabi",
        "armv7-linux-androideabi" => "armv7a-linux-androideabi",
        other => other,
    }
}

pub fn to_env_key(target: &TargetInfo<'static>) -> String {
    target.triple.replace("-", "_").to_lowercase()
}

pub fn to_cargo_env_key(target: &TargetInfo<'static>) -> String {
    target.triple.replace("-", "_").to_uppercase()
}

fn toolchain_triple(target: &TargetInfo<'static>) -> &'static str {
    match target.triple {
        "armv7-linux-androideabi" => "arm-linux-androideabi",
        other => other,
    }
}

fn toolchain_suffix(target: &TargetInfo<'static>, host_arch: &str, bin: &str) -> PathBuf {
    [
        "toolchains",
        "llvm",
        "prebuilt",
        host_arch,
        "bin",
        &format!("{}-{}", toolchain_triple(target), bin),
    ]
    .iter()
    .collect()
}

pub fn test_command(sdk: &AndroidSdk, requested: &Options) -> TaiResult<Command> {
    let cc_key = format!("CC_{}", to_env_key(&requested.target));
    let ar_key = format!("AR_{}", to_env_key(&requested.target));
    let cxx_key = format!("CXX_{}", to_env_key(&requested.target));

    // https://github.com/rust-lang/rustup/blob/master/ci/docker/android/Dockerfile
    let cargo_ar_key = format!(
        "CARGO_TARGET_{}_{}",
        to_cargo_env_key(&requested.target),
        "AR"
    );
    let cargo_linker_key = format!(
        "CARGO_TARGET_{}_{}",
        to_cargo_env_key(&requested.target),
        "LINKER"
    );

    let target_ar = &sdk
        .ndk
        .join(toolchain_suffix(&requested.target, &HOST_ARCH, "ar"));
    let target_linker = &sdk.ndk.join(clang_suffix(
        &requested.target,
        &HOST_ARCH,
        requested.android_platform,
        "",
    ));
    let target_cxx = &sdk.ndk.join(clang_suffix(
        &requested.target,
        &HOST_ARCH,
        requested.android_platform,
        "++",
    ));

    debug!("{}={}", cc_key, target_linker.display());
    debug!("{}={}", ar_key, target_ar.display());
    debug!("{}={}", cxx_key, target_cxx.display());
    debug!("{}={}", cargo_ar_key, target_ar.display());
    debug!("{}={}", cargo_linker_key, target_linker.display());

    let mut cmd = Command::new("cargo");
    cmd.env(cc_key, &target_linker)
        .env(ar_key, &target_ar)
        .env(cxx_key, &target_cxx)
        .env(cargo_ar_key, &target_ar)
        .env(cargo_linker_key, &target_linker);
    cmd.args(&["build", "--tests"]);

    Ok(cmd)
}

// pub(crate) fn strip(ndk_home: &Path, triple: &str, bin_path: &Path) -> std::process::ExitStatus {
//     let target_strip = Path::new(&ndk_home).join(toolchain_suffix(&triple, &HOST_ARCH, "strip"));

//     tracing::debug!("strip: {}", &target_strip.display());

//     Command::new(target_strip)
//         .arg(&bin_path)
//         .status()
//         .expect("strip crashed")
// }
