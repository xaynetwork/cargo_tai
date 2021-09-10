use std::{
    convert::{TryFrom, TryInto},
    path::PathBuf,
};

use anyhow::anyhow;

use crate::{
    bundle::BuildBundles,
    compiler::BuildUnit,
    ios::{bundle::signing::SigningSettings, tools::ios_deploy},
    options::{self, GeneralOptions},
    TaiResult,
};

pub struct Context {
    pub requested: Options,
    pub devices: Option<Vec<ios_deploy::Device>>,
    pub simulators: Option<Vec<simctl::Device>>,
    pub build_units: Option<Vec<BuildUnit>>,
    pub signing_settings: Option<SigningSettings>,
    pub build_bundles: Option<BuildBundles>,
}

impl Context {
    pub fn new(requested: options::Options) -> TaiResult<Self> {
        Ok(Self {
            requested: requested.try_into()?,
            devices: None,
            simulators: None,
            build_units: None,
            signing_settings: None,
            build_bundles: None,
        })
    }

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
        self.requested
            .mobile_provision
            .as_ref()
            .ok_or_else(|| anyhow!("no mobile provision found"))
    }
}

pub struct Options {
    pub general: GeneralOptions,

    pub mobile_provision: Option<PathBuf>,
}

impl TryFrom<options::Options> for Options {
    type Error = anyhow::Error;

    fn try_from(opt: options::Options) -> Result<Self, Self::Error> {
        Ok(Self {
            general: opt.general,
            mobile_provision: opt.platform.ios_mobile_provision,
        })
    }
}
