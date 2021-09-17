use crate::{common::task::Task, ios::bundle::signing::find_signing_settings, TaiResult};

use super::Context;

pub struct ReadSigningSettings;

impl Task<Context> for ReadSigningSettings {
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        if let Ok(mobile_provision) = context.mobile_provision() {
            let sig_settings = find_signing_settings(mobile_provision)?;
            context.signing_settings = Some(sig_settings);
        }

        Ok(context)
    }
}
