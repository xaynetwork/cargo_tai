use anyhow::anyhow;

use crate::{android::tools::AndroidSdk, common::task::Task, TaiResult};

use super::Context;

pub struct FindAndroidSdk;

impl Task<Context> for FindAndroidSdk {
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let sdk = AndroidSdk::derive_sdk(
            &context
                .opts
                .android
                .as_ref()
                .ok_or_else(|| anyhow!("no ndk"))?
                .ndk,
        )?;

        context.android_sdk = Some(sdk);

        Ok(context)
    }
}
