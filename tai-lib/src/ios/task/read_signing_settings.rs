use anyhow::anyhow;

use crate::{ios::bundle::signing::find_signing_settings, task::Task, TaiResult};

use super::Context;

pub struct ReadSigningSettings;

impl Task for ReadSigningSettings {
    type Context = Context;

    fn run(&self, mut context: Self::Context) -> TaiResult<Self::Context> {
        let device = context
            .devices
            .as_ref()
            .map(|d| d.first())
            .flatten()
            .ok_or_else(|| anyhow!("no devices"))?;

        let mobile_provision = &context
            .requested
            .platform
            .ios_mobile_provision
            .as_ref()
            .ok_or_else(|| anyhow!("the option mobile_provision is missing"))?;

        let sig_settings = find_signing_settings(&device.id, mobile_provision)?;
        context.signing_settings = Some(sig_settings);
        Ok(context)
    }
}
