use std::convert::TryFrom;

use anyhow::{anyhow, bail};
use cfg_expr::targets::TargetInfo;
use tracing::instrument;

use crate::{
    common::{opts::Options, task::Task},
    ios::bundle::signing::find_signing_settings,
    TaiResult,
};

use super::Context;

#[derive(Debug)]
pub struct ReadSigningSettings;

impl Task<Context> for ReadSigningSettings {
    #[instrument(name = "Read Signing Settings", skip_all)]
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let opts: &Options = context.get();
        let maybe_mobile_provision = opts
            .ios
            .as_ref()
            .map(|ios| &ios.mobile_provision)
            .ok_or_else(|| anyhow!("Building for iphoneos requires a mobile provision file"));

        if let Sdk::IPhoneOS = Sdk::try_from(&opts.compiler.target)? {
            // for IPhoneOS we require a mobile_provision
            let sig_settings = find_signing_settings(maybe_mobile_provision?)?;
            context.insert(sig_settings);
        } else {
            // for IPhoneSimulator it can be optional
            if let Ok(mobile_provision) = maybe_mobile_provision {
                let sig_settings = find_signing_settings(mobile_provision)?;
                context.insert(sig_settings);
            }
        }

        Ok(context)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Sdk {
    IPhoneOS,
    IPhoneSimulator,
}

impl TryFrom<&TargetInfo<'_>> for Sdk {
    type Error = anyhow::Error;

    fn try_from(target: &TargetInfo<'_>) -> Result<Self, Self::Error> {
        match target.triple {
            "aarch64-apple-ios" => Ok(Sdk::IPhoneOS),
            "x86_64-apple-ios" => Ok(Sdk::IPhoneSimulator),
            _ => bail!("Unsupported target `{}`", target.triple),
        }
    }
}
