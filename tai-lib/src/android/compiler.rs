use std::{path::PathBuf, process::Command};

use anyhow::{bail, Context};
use cfg_expr::targets::TargetInfo;
use tracing::debug;

use crate::TaiResult;

use super::{task::build_buildunits::Options, tools::AndroidSdk};

#[cfg(target_os = "macos")]
const HOST_ARCH: &str = "darwin-x86_64";
#[cfg(target_os = "linux")]
const HOST_ARCH: &str = "linux-x86_64";

pub fn benches_command(sdk: &AndroidSdk, requested: &Options) -> TaiResult<Command> {
    let mut cmd = setup_android_deps(sdk, requested)?;
    cmd.args(&["build", "--release", "--benches"]);
    Ok(cmd)
}

pub fn tests_command(sdk: &AndroidSdk, requested: &Options) -> TaiResult<Command> {
    let mut cmd = setup_android_deps(sdk, requested)?;
    cmd.args(&["build", "--tests"]);
    Ok(cmd)
}

pub fn bench_command(sdk: &AndroidSdk, requested: &Options) -> TaiResult<Command> {
    let mut cmd = setup_android_deps(sdk, requested)?;
    cmd.args(&["build", "--release", "--bench"]);
    Ok(cmd)
}

pub fn test_command(sdk: &AndroidSdk, requested: &Options) -> TaiResult<Command> {
    let mut cmd = setup_android_deps(sdk, requested)?;
    cmd.args(&["build", "--test"]);
    Ok(cmd)
}

pub fn build_lib_command(sdk: &AndroidSdk, requested: &Options) -> TaiResult<Command> {
    let mut cmd = setup_android_deps(sdk, requested)?;
    cmd.args(&["build"]);
    Ok(cmd)
}

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

fn to_clang_suffix(target: &TargetInfo<'static>) -> &'static str {
    match target.triple {
        "arm-linux-androideabi" => "armv7a-linux-androideabi",
        "armv7-linux-androideabi" => "armv7a-linux-androideabi",
        other => other,
    }
}

fn to_env_key(target: &TargetInfo<'static>) -> String {
    target.triple.replace("-", "_").to_lowercase()
}

fn to_cargo_env_key(target: &TargetInfo<'static>) -> String {
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

fn setup_android_deps(sdk: &AndroidSdk, requested: &Options) -> TaiResult<Command> {
    let cc_key = format!("CC_{}", to_env_key(&requested.general.compiler.target));
    let ar_key = format!("AR_{}", to_env_key(&requested.general.compiler.target));
    let cxx_key = format!("CXX_{}", to_env_key(&requested.general.compiler.target));

    // https://github.com/rust-lang/rustup/blob/master/ci/docker/android/Dockerfile
    let cargo_ar_key = format!(
        "CARGO_TARGET_{}_{}",
        to_cargo_env_key(&requested.general.compiler.target),
        "AR"
    );
    let cargo_linker_key = format!(
        "CARGO_TARGET_{}_{}",
        to_cargo_env_key(&requested.general.compiler.target),
        "LINKER"
    );

    let target_ar = &sdk.ndk.join(toolchain_suffix(
        &requested.general.compiler.target,
        HOST_ARCH,
        "ar",
    ));

    let target_linker = &sdk.ndk.join(clang_suffix(
        &requested.general.compiler.target,
        HOST_ARCH,
        requested.android_api_lvl,
        "",
    ));

    let target_cxx = &sdk.ndk.join(clang_suffix(
        &requested.general.compiler.target,
        HOST_ARCH,
        requested.android_api_lvl,
        "++",
    ));

    check_if_utils_exists(&[target_ar, target_linker, target_cxx])
        .with_context(|| "Did you choose the right Android API level?")?;

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

    Ok(cmd)
}

fn check_if_utils_exists(paths: &[&PathBuf]) -> TaiResult<()> {
    paths.iter().try_for_each(|path| {
        if !path.exists() {
            bail!("{} does not exist", path.display());
        }
        Ok(())
    })
}

// pub(crate) fn strip(ndk_home: &Path, triple: &str, bin_path: &Path) -> std::process::ExitStatus {
//     let target_strip = Path::new(&ndk_home).join(toolchain_suffix(&triple, &HOST_ARCH, "strip"));

//     tracing::debug!("strip: {}", &target_strip.display());

//     Command::new(target_strip)
//         .arg(&bin_path)
//         .status()
//         .expect("strip crashed")
// }
