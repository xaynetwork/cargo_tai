use std::path::PathBuf;

use anyhow::anyhow;

use crate::{
    common::{
        bundle::BuildBundles,
        compiler::BuildUnit,
        options::{BinaryOptions, BuildOptions, Options},
        project::ProjectMetadata,
    },
    ios::{bundle::signing::SigningSettings, tools::ios_deploy},
    TaiResult,
};

use super::create_xcode_project::XCodeProject;

pub struct Context {
    pub options: Options,
    pub devices: Option<Vec<ios_deploy::Device>>,
    pub simulators: Option<Vec<simctl::Device>>,
    pub build_units: Option<Vec<BuildUnit>>,
    pub signing_settings: Option<SigningSettings>,
    pub build_bundles: Option<BuildBundles>,
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

    pub fn take_build_units(&mut self) -> TaiResult<Vec<BuildUnit>> {
        self.build_units
            .take()
            .ok_or_else(|| anyhow!("no build units found"))
    }

    pub fn signing_settings(&self) -> TaiResult<&SigningSettings> {
        self.signing_settings
            .as_ref()
            .ok_or_else(|| anyhow!("no signing settings found"))
    }

    pub fn build_bundles(&self) -> TaiResult<&BuildBundles> {
        self.build_bundles
            .as_ref()
            .ok_or_else(|| anyhow!("no build bundles found"))
    }

    pub fn mobile_provision(&self) -> TaiResult<&PathBuf> {
        Ok(&self
            .options
            .ios
            .as_ref()
            .ok_or_else(|| anyhow!("no mobile provision found"))?
            .mobile_provision)
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
    fn from(options: Options) -> Self {
        Self {
            options,
            devices: None,
            simulators: None,
            build_units: None,
            signing_settings: None,
            build_bundles: None,
            project_metadata: None,
            xcode_project: None,
            xcode_product: None,
            xcode_test_product: None,
        }
    }
}
