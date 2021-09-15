use anyhow::anyhow;

use crate::{
    android::tools::{adb::Device, AndroidSdk},
    common::{
        bundle::BuildBundles,
        compiler::BuildUnit,
        options::{BinaryOptions, BuildOptions, Options},
        project::ProjectMetadata,
    },
    TaiResult,
};

pub struct Context {
    pub options: Options,
    pub android_sdk: Option<AndroidSdk>,
    pub devices: Option<Vec<Device>>,
    pub build_units: Option<Vec<BuildUnit>>,
    pub build_bundles: Option<BuildBundles>,
    pub project_metadata: Option<ProjectMetadata>,
}

impl Context {
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

    pub fn binary(&self) -> TaiResult<&BinaryOptions> {
        self.options
            .binary
            .as_ref()
            .ok_or_else(|| anyhow!("no binary found"))
    }

    pub fn build(&self) -> TaiResult<&BuildOptions> {
        self.options
            .build
            .as_ref()
            .ok_or_else(|| anyhow!("no build found"))
    }

    pub fn project_metadata(&self) -> TaiResult<&ProjectMetadata> {
        self.project_metadata
            .as_ref()
            .ok_or_else(|| anyhow!("no project metadata found"))
    }
}

impl From<Options> for Context {
    fn from(options: Options) -> Self {
        Self {
            options,
            devices: None,
            build_units: None,
            build_bundles: None,
            project_metadata: None,
            android_sdk: None,
        }
    }
}
