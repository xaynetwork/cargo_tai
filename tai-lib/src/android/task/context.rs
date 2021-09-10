use anyhow::anyhow;

use crate::{
    android::tools::{adb::Device, AndroidSdk},
    bundle::BuildBundles,
    compiler::BuildUnit,
    options::Options,
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
    pub fn new(requested: Options) -> Self {
        Self {
            requested,
            android_sdk: None,
            devices: None,
            build_units: None,
            build_bundles: None,
        }
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
