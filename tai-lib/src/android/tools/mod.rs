use std::path::PathBuf;

use anyhow::anyhow;

use crate::{common::opts::AndroidOptions, TaiResult};

pub mod adb;

#[derive(Debug)]
pub struct AndroidEnv {
    pub adb: PathBuf,
    pub ndk: PathBuf,
    pub sdk: PathBuf,
}

impl AndroidEnv {
    pub fn derive_env(opts: &AndroidOptions) -> TaiResult<AndroidEnv> {
        let ndk = opts.ndk.to_path_buf();
        let sdk = opts
            .sdk
            .clone()
            .or_else(|| ndk.parent().and_then(|p| p.parent()).map(Into::into))
            .ok_or_else(|| anyhow!("Failed to find SDK folder in `../../{}`", ndk.display()))?;

        let adb = sdk.join("platform-tools").join("adb");
        Ok(Self { adb, ndk, sdk })
    }
}
