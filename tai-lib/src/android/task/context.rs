use anyhow::anyhow;

use crate::{
    android::tools::{adb::Device, AndroidSdk},
    common::{
        bundle::BuiltBundles,
        compiler::BuiltUnit,
        opts::{BinaryOptions, Options},
        project::ProjectMetadata,
    },
    TaiResult,
};

pub struct Context {
    pub opts: Options,
    pub android_sdk: Option<AndroidSdk>,
    pub devices: Option<Vec<Device>>,
    pub built_units: Option<Vec<BuiltUnit>>,
    pub built_bundles: Option<BuiltBundles>,
    pub project_metadata: Option<ProjectMetadata>,
}

impl Context {
    pub fn devices(&self) -> TaiResult<&Vec<Device>> {
        self.devices
            .as_ref()
            .ok_or_else(|| anyhow!("no iOS devices found"))
    }

    pub fn take_built_units(&mut self) -> TaiResult<Vec<BuiltUnit>> {
        self.built_units
            .take()
            .ok_or_else(|| anyhow!("no built units found"))
    }

    pub fn built_bundles(&self) -> TaiResult<&BuiltBundles> {
        self.built_bundles
            .as_ref()
            .ok_or_else(|| anyhow!("no built bundles found"))
    }

    pub fn android_sdk(&self) -> TaiResult<&AndroidSdk> {
        self.android_sdk
            .as_ref()
            .ok_or_else(|| anyhow!("no android SDK found"))
    }

    pub fn binary(&self) -> TaiResult<&BinaryOptions> {
        self.opts
            .binary
            .as_ref()
            .ok_or_else(|| anyhow!("no binary found"))
    }

    pub fn project_metadata(&self) -> TaiResult<&ProjectMetadata> {
        self.project_metadata
            .as_ref()
            .ok_or_else(|| anyhow!("no project metadata found"))
    }

    pub fn built_units(&self) -> TaiResult<&Vec<BuiltUnit>> {
        self.built_units
            .as_ref()
            .ok_or_else(|| anyhow!("no built units found"))
    }
}

impl From<Options> for Context {
    fn from(opts: Options) -> Self {
        Self {
            opts,
            devices: None,
            built_units: None,
            built_bundles: None,
            project_metadata: None,
            android_sdk: None,
        }
    }
}
