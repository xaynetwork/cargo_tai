use std::{env, path::PathBuf};

use anyhow::anyhow;

use crate::TaiResult;

pub mod adb;

pub struct AndroidSdk {
    pub adb: PathBuf,
    pub ndk: PathBuf,
}

impl AndroidSdk {
    pub fn derive_sdk() -> TaiResult<AndroidSdk> {
        let ndk_home =
            env::var_os("ANDROID_NDK_HOME").ok_or(anyhow!("ANDROID_NDK_HOME not set"))?;

        let ndk = PathBuf::from(ndk_home);

        let adb = ndk
            .parent()
            .ok_or(anyhow!(
                "failed to find `ndk` folder in ../ANDROID_NDK_HOME"
            ))?
            .parent()
            .ok_or(anyhow!(
                "failed to find `sdk` folder in ../../ANDROID_NDK_HOME"
            ))?
            .join("platform-tools")
            .join("adb")
            .to_path_buf();

        Ok(Self { adb, ndk })
    }
}
