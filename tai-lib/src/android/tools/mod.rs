use std::path::{Path, PathBuf};

use anyhow::anyhow;

use crate::TaiResult;

pub mod adb;

pub struct AndroidSdk {
    pub adb: PathBuf,
    pub ndk: PathBuf,
    pub sdk: PathBuf,
}

impl AndroidSdk {
    pub fn derive_sdk<P: AsRef<Path>>(ndk: P) -> TaiResult<AndroidSdk> {
        let sdk = ndk
            .as_ref()
            .parent()
            .and_then(|p| p.parent())
            .ok_or_else(|| anyhow!("failed to find `sdk` folder in ../../ANDROID_NDK_HOME"))?;

        let adb = sdk.join("platform-tools").join("adb");

        Ok(Self {
            adb,
            ndk: ndk.as_ref().to_path_buf(),
            sdk: sdk.to_path_buf(),
        })
    }
}
