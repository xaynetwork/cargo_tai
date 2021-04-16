use std::path::{Path, PathBuf};

use anyhow::anyhow;

use crate::TaiResult;

pub mod adb;

pub struct AndroidSdk {
    pub adb: PathBuf,
    pub ndk: PathBuf,
}

impl AndroidSdk {
    pub fn derive_sdk<P: AsRef<Path>>(ndk: P) -> TaiResult<AndroidSdk> {
        let adb = ndk
            .as_ref()
            .parent()
            .map(|p| p.parent())
            .flatten()
            .ok_or_else(|| anyhow!("failed to find `sdk` folder in ../../ANDROID_NDK_HOME"))?
            .join("platform-tools")
            .join("adb");

        Ok(Self {
            adb,
            ndk: ndk.as_ref().to_path_buf(),
        })
    }
}
