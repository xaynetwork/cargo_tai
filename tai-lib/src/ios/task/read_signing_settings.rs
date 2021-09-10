use anyhow::anyhow;

use crate::{ios::bundle::signing::find_signing_settings, task::Task, TaiResult};

use super::Context;

pub struct ReadSigningSettings;

impl Task for ReadSigningSettings {
    type Context = Context;

    fn run(&self, mut context: Self::Context) -> TaiResult<Self::Context> {
        let device = context
            .devices()?
            .first()
            .ok_or_else(|| anyhow!("no devices"))?;

        let mobile_provision = context.mobile_provision()?;

        let sig_settings = find_signing_settings(&device.id, mobile_provision)?;
        context.signing_settings = Some(sig_settings);
        Ok(context)
    }
}
