use std::convert::TryFrom;

use tracing::instrument;

use crate::{
    common::task::Task,
    ios::{bundle::signing::find_signing_settings, tools::xcodebuild::Sdk},
    TaiResult,
};

use super::Context;

pub struct ReadSigningSettings;

impl Task<Context> for ReadSigningSettings {
    #[instrument(name = "read_signing_settings", skip(self, context))]
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let maybe_mobile_provision = context.mobile_provision();

        if let Sdk::IPhoneOS = Sdk::try_from(&context.options.compiler.target)? {
            // for IPhoneOS we require a mobile_provision
            let sig_settings = find_signing_settings(maybe_mobile_provision?)?;
            context.signing_settings = Some(sig_settings);
        } else {
            // for IPhoneSimulator it is optional
            if let Ok(mobile_provision) = maybe_mobile_provision {
                let sig_settings = find_signing_settings(mobile_provision)?;
                context.signing_settings = Some(sig_settings);
            }
        }

        Ok(context)
    }
}
