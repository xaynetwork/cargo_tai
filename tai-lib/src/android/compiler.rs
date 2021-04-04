use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::anyhow;
use tracing::debug;

use crate::{task::Options, TaiResult};

#[cfg(target_os = "macos")]
const ARCH: &str = "darwin-x86_64";
#[cfg(target_os = "linux")]
const ARCH: &str = "linux-x86_64";

// ~/Library/Android/sdk/ndk/<22.1.7171670>/toolchains/llvm/prebuilt/<darwin-x86_64>/<x86_64-linux-android>/bin/

fn clang_suffix(triple: &str, arch: &str, platform: u8, postfix: &str) -> PathBuf {
    let tool_triple = match triple {
        "arm-linux-androideabi" => "armv7a-linux-androideabi",
        "armv7-linux-androideabi" => "armv7a-linux-androideabi",
        _ => triple,
    };

    [
        "toolchains",
        "llvm",
        "prebuilt",
        arch,
        "bin",
        &format!("{}{}-clang{}", tool_triple, platform, postfix),
    ]
    .iter()
    .collect()
}

fn toolchain_triple(triple: &str) -> &str {
    match triple {
        "armv7-linux-androideabi" => "arm-linux-androideabi",
        _ => triple,
    }
}

fn toolchain_suffix(triple: &str, arch: &str, bin: &str) -> PathBuf {
    [
        "toolchains",
        "llvm",
        "prebuilt",
        arch,
        "bin",
        &format!("{}-{}", toolchain_triple(triple), bin),
    ]
    .iter()
    .collect()
}

fn cargo_env_target_cfg(triple: &str, key: &str) -> String {
    format!("CARGO_TARGET_{}_{}", &triple.replace("-", "_"), key).to_uppercase()
}

pub(crate) fn strip(ndk_home: &Path, triple: &str, bin_path: &Path) -> std::process::ExitStatus {
    let target_strip = Path::new(&ndk_home).join(toolchain_suffix(&triple, &ARCH, "strip"));

    tracing::debug!("strip: {}", &target_strip.display());

    Command::new(target_strip)
        .arg(&bin_path)
        .status()
        .expect("strip crashed")
}

pub fn test_command(requested: &Options) -> TaiResult<Command> {
    let triple = requested.target.triple;
    let sdk = derive_sdk()?;
    let platform = 21;

    let target_ar = Path::new(&sdk.ndk).join(toolchain_suffix(&triple, &ARCH, "ar"));
    let target_linker = Path::new(&sdk.ndk).join(clang_suffix(&triple, &ARCH, platform, ""));
    let target_cxx = Path::new(&sdk.ndk).join(clang_suffix(&triple, &ARCH, platform, "++"));

    let cc_key = format!("CC_{}", &triple.to_uppercase());
    let ar_key = format!("AR_{}", &triple.to_uppercase());
    let cxx_key = format!("CXX_{}", &triple.to_uppercase());
    let cargo_ar_key = cargo_env_target_cfg(&triple, "ar");
    let cargo_cxx_key = cargo_env_target_cfg(&triple, "linker");

    debug!("{}={}", cc_key, target_linker.display());
    debug!("{}={}", ar_key, target_ar.display());
    debug!("{}={}", cxx_key, target_cxx.display());
    debug!("{}={}", cargo_ar_key, target_ar.display());
    debug!("{}={}", cargo_cxx_key, target_linker.display());

    let mut cmd = Command::new("cargo");
    cmd.env(cc_key, &target_linker)
        .env(ar_key, &target_ar)
        .env(cxx_key, &target_cxx)
        .env(cargo_ar_key, &target_ar)
        .env(cargo_cxx_key, &target_linker);
    cmd.args(&["build", "--tests"]);

    Ok(cmd)
}

pub struct AndroidSdk {
    sdk: PathBuf,
    ndk: PathBuf,
}

fn derive_sdk() -> TaiResult<AndroidSdk> {
    let ndk_home = env::var_os("ANDROID_NDK_HOME").ok_or(anyhow!("ANDROID_NDK_HOME not set"))?;

    let ndk = PathBuf::from(ndk_home);
    let sdk = ndk
        .parent()
        .ok_or(anyhow!("ANDROID_NDK_HOME not set"))?
        .parent()
        .ok_or(anyhow!("ANDROID_NDK_HOME not set"))?
        .to_path_buf();

    Ok(AndroidSdk { sdk, ndk })
}

// [2021-04-04T17:15:11Z DEBUG cargo_ndk::cargo] ar: /Users/robert/Library/Android/sdk/ndk/22.1.7171670/toolchains/llvm/prebuilt/darwin-x86_64/bin/x86_64-linux-android-ar
// [2021-04-04T17:15:11Z DEBUG cargo_ndk::cargo] linker: /Users/robert/Library/Android/sdk/ndk/22.1.7171670/toolchains/llvm/prebuilt/darwin-x86_64/bin/x86_64-linux-android21-clang
// [2021-04-04T17:15:11Z DEBUG cargo_ndk::cargo] cargo: /Users/robert/.rustup/toolchains/1.49.0-x86_64-apple-darwin/bin/cargo

// rustup target add armv7-linux-androideabi   -> arm-linux-androideabi
// rustup target add i686-linux-android        -> i686-linux-android
// rustup target add x86_64-linux-android      -> x86_64-linux-android
// rustup target add aarch64-linux-android     -> aarch64-linux-android
