use std::convert::TryFrom;

use anyhow::bail;
use cfg_expr::targets::TargetInfo;
use tracing::instrument;

use crate::{common::task::Task, ios::bundle::signing::find_signing_settings, TaiResult};

use super::Context;

pub struct ReadSigningSettings;

impl Task<Context> for ReadSigningSettings {
    #[instrument(name = "read_signing_settings", skip(self, context))]
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let maybe_mobile_provision = context.mobile_provision();

        if let Sdk::IPhoneOS = Sdk::try_from(&context.opts.compiler.target)? {
            // for IPhoneOS we require a mobile_provision
            let sig_settings = find_signing_settings(maybe_mobile_provision?)?;
            context.signing_settings = Some(sig_settings);
        } else {
            // for IPhoneSimulator it can be optional
            if let Ok(mobile_provision) = maybe_mobile_provision {
                let sig_settings = find_signing_settings(mobile_provision)?;
                context.signing_settings = Some(sig_settings);
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

    fn try_from(value: &TargetInfo<'_>) -> Result<Self, Self::Error> {
        match value.triple {
            "aarch64-apple-ios" => Ok(Sdk::IPhoneOS),
            "x86_64-apple-ios" => Ok(Sdk::IPhoneSimulator),
            _ => bail!("unsupported target"),
        }
    }
}
