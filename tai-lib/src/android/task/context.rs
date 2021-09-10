use std::{
    convert::{TryFrom, TryInto},
    path::PathBuf,
};

use anyhow::anyhow;

use crate::{
    android::tools::{adb::Device, AndroidSdk},
    bundle::BuildBundles,
    compiler::BuildUnit,
    options::{self, GeneralOptions},
    TaiResult,
};

pub struct Context {
    pub requested: Options,
    pub android_sdk: Option<AndroidSdk>,
    pub devices: Option<Vec<Device>>,
    pub build_units: Option<Vec<BuildUnit>>,
    pub build_bundles: Option<BuildBundles>,
}

impl Context {
    pub fn new(requested: options::Options) -> TaiResult<Self> {
        Ok(Self {
            requested: requested.try_into()?,
            android_sdk: None,
            devices: None,
            build_units: None,
            build_bundles: None,
        })
    }

    pub fn devices(&self) -> TaiResult<&Vec<Device>> {
        self.devices
            .as_ref()
            .ok_or_else(|| anyhow!("no iOS devices found"))
    }

    pub fn take_build_units(&mut self) -> TaiResult<Vec<BuildUnit>> {
        self.build_units
            .take()
            .ok_or_else(|| anyhow!("no build units found"))
    }

    pub fn build_bundles(&self) -> TaiResult<&BuildBundles> {
        self.build_bundles
            .as_ref()
            .ok_or_else(|| anyhow!("no build bundles found"))
    }

    pub fn android_sdk(&self) -> TaiResult<&AndroidSdk> {
        self.android_sdk
            .as_ref()
            .ok_or_else(|| anyhow!("no android SDK found"))
    }
}

pub struct Options {
    pub general: GeneralOptions,

    pub android_api_lvl: u8,
    pub android_ndk: PathBuf,
}

impl TryFrom<options::Options> for Options {
    type Error = anyhow::Error;

    fn try_from(opt: options::Options) -> Result<Self, Self::Error> {
        Ok(Self {
            general: opt.general,
            android_api_lvl: opt
                .platform
                .android_api_lvl
                .ok_or_else(|| anyhow!("the option android_api_lvl is missing"))?,
            android_ndk: opt
                .platform
                .android_ndk
                .ok_or_else(|| anyhow!("the option android_ndk is missing"))?,
        })
    }
}
