use anyhow::anyhow;

use crate::{common::task::Task, ios::bundle::signing::find_signing_settings, TaiResult};

use super::Context;

pub struct ReadSigningSettings;

impl Task<Context> for ReadSigningSettings {
    fn run(&self, mut context: Context) -> TaiResult<Context> {
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
