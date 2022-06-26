use std::process::Command;

use tracing::debug;

use crate::{common::opts::Options, TaiResult};

use super::tools::AndroidSdk;

pub fn benches_command(sdk: &AndroidSdk, requested: &Options) -> TaiResult<Command> {
    let mut cmd = setup_cargo_ndk(sdk, requested)?;
    cmd.args(&["--benches"]);
    Ok(cmd)
}

pub fn tests_command(sdk: &AndroidSdk, requested: &Options) -> TaiResult<Command> {
    let mut cmd = setup_cargo_ndk(sdk, requested)?;
    cmd.args(&["--tests"]);
    Ok(cmd)
}

pub fn bench_command(sdk: &AndroidSdk, requested: &Options) -> TaiResult<Command> {
    let mut cmd = setup_cargo_ndk(sdk, requested)?;
    cmd.args(&["--bench"]);
    Ok(cmd)
}

pub fn test_command(sdk: &AndroidSdk, requested: &Options) -> TaiResult<Command> {
    let mut cmd = setup_cargo_ndk(sdk, requested)?;
    cmd.args(&["--test"]);
    Ok(cmd)
}

fn setup_cargo_ndk(sdk: &AndroidSdk, requested: &Options) -> TaiResult<Command> {
    let ndk_home_key = "ANDROID_NDK_HOME";
    let sdk_home_key = "ANDROID_SDK_HOME";

    debug!("{}={}", ndk_home_key, sdk.ndk.display());
    debug!("{}={}", sdk_home_key, sdk.sdk.display());

    let mut cmd = Command::new("cargo");
    cmd.args(&["ndk", "-t", requested.compiler.target.triple, "build"])
        .env(ndk_home_key, &sdk.ndk)
        .env(sdk_home_key, &sdk.sdk);

    Ok(cmd)
}
