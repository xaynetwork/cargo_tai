use std::path::PathBuf;

use anyhow::anyhow;

use crate::{
    common::{
        bundle::BuiltBundles,
        compiler::BuiltUnit,
        opts::{BinaryOptions, BuildOptions, Options},
        project::ProjectMetadata,
    },
    ios::{bundle::signing::SigningSettings, tools::ios_deploy},
    TaiResult,
};

use super::create_xcode_project::XCodeProject;

pub struct Context {
    pub opts: Options,
    pub devices: Option<Vec<ios_deploy::Device>>,
    pub simulators: Option<Vec<simctl::Device>>,
    pub built_units: Option<Vec<BuiltUnit>>,
    pub signing_settings: Option<SigningSettings>,
    pub built_bundles: Option<BuiltBundles>,
    pub project_metadata: Option<ProjectMetadata>,
    pub xcode_project: Option<XCodeProject>,
    pub xcode_product: Option<PathBuf>,
    pub xcode_test_product: Option<PathBuf>,
}

impl Context {
    pub fn devices(&self) -> TaiResult<&Vec<ios_deploy::Device>> {
        self.devices
            .as_ref()
            .ok_or_else(|| anyhow!("no iOS devices found"))
    }

    pub fn simulators(&self) -> TaiResult<&Vec<simctl::Device>> {
        self.simulators
            .as_ref()
            .ok_or_else(|| anyhow!("no iOS simulators found"))
    }

    pub fn take_built_units(&mut self) -> TaiResult<Vec<BuiltUnit>> {
        self.built_units
            .take()
            .ok_or_else(|| anyhow!("no built units found"))
    }

    pub fn built_units(&self) -> TaiResult<&Vec<BuiltUnit>> {
        self.built_units
            .as_ref()
            .ok_or_else(|| anyhow!("no built units found"))
    }

    pub fn signing_settings(&self) -> TaiResult<&SigningSettings> {
        self.signing_settings
            .as_ref()
            .ok_or_else(|| anyhow!("no signing settings found"))
    }

    pub fn built_bundles(&self) -> TaiResult<&BuiltBundles> {
        self.built_bundles
            .as_ref()
            .ok_or_else(|| anyhow!("no build bundles found"))
    }

    pub fn mobile_provision(&self) -> TaiResult<&PathBuf> {
        Ok(&self
            .opts
            .ios
            .as_ref()
            .ok_or_else(|| anyhow!("no mobile provision found"))?
            .mobile_provision)
    }

    pub fn binary(&self) -> TaiResult<&BinaryOptions> {
        self.opts
            .binary
            .as_ref()
            .ok_or_else(|| anyhow!("no binary found"))
    }

    pub fn build(&self) -> TaiResult<&BuildOptions> {
        self.opts
            .build
            .as_ref()
            .ok_or_else(|| anyhow!("no build found"))
    }

    pub fn project_metadata(&self) -> TaiResult<&ProjectMetadata> {
        self.project_metadata
            .as_ref()
            .ok_or_else(|| anyhow!("no project metadata found"))
    }

    pub fn xcode_project(&self) -> TaiResult<&XCodeProject> {
        self.xcode_project
            .as_ref()
            .ok_or_else(|| anyhow!("no xcode project found"))
    }

    pub fn xcode_product(&self) -> TaiResult<&PathBuf> {
        self.xcode_product
            .as_ref()
            .ok_or_else(|| anyhow!("no xcode product found"))
    }

    pub fn xcode_test_product(&self) -> TaiResult<&PathBuf> {
        self.xcode_test_product
            .as_ref()
            .ok_or_else(|| anyhow!("no xcode test product found"))
    }
}

impl From<Options> for Context {
    fn from(opts: Options) -> Self {
        Self {
            opts,
            devices: None,
            simulators: None,
            built_units: None,
            signing_settings: None,
            built_bundles: None,
            project_metadata: None,
            xcode_project: None,
            xcode_product: None,
            xcode_test_product: None,
        }
    }
}
