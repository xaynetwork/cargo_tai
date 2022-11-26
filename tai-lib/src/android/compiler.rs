use std::process::Command;

use tracing::debug;

use crate::{
    common::opts::{AndroidOptions, Options},
    TaiResult,
};

use super::tools::AndroidEnv;

pub fn benches_command(env: &AndroidEnv, requested: &Options) -> TaiResult<Command> {
    let mut cmd = setup_cargo_ndk(env, requested)?;
    cmd.args(["--benches"]);
    Ok(cmd)
}

pub fn tests_command(env: &AndroidEnv, requested: &Options) -> TaiResult<Command> {
    let mut cmd = setup_cargo_ndk(env, requested)?;
    cmd.args(["--tests"]);
    Ok(cmd)
}

pub fn bench_command(env: &AndroidEnv, requested: &Options) -> TaiResult<Command> {
    let mut cmd = setup_cargo_ndk(env, requested)?;
    cmd.args(["--bench"]);
    Ok(cmd)
}

pub fn test_command(env: &AndroidEnv, requested: &Options) -> TaiResult<Command> {
    let mut cmd = setup_cargo_ndk(env, requested)?;
    cmd.args(["--test"]);
    Ok(cmd)
}

fn setup_cargo_ndk(env: &AndroidEnv, requested: &Options) -> TaiResult<Command> {
    const NDK_HOME_KEY: &str = "ANDROID_NDK_HOME";
    const SDK_HOME_KEY: &str = "ANDROID_SDK_HOME";

    debug!("{}={}", NDK_HOME_KEY, env.ndk.display());
    debug!("{}={}", SDK_HOME_KEY, env.sdk.display());

    let mut cmd = Command::new("cargo");
    cmd.env(NDK_HOME_KEY, &env.ndk)
        .env(SDK_HOME_KEY, &env.sdk)
        .args(["ndk", "-t", requested.compiler.target.triple]);
    if let Some(AndroidOptions {
        cargo_ndk_args: Some(args),
        ..
    }) = requested.android.as_ref()
    {
        cmd.args(args);
    }
    cmd.arg("build");

    Ok(cmd)
}
