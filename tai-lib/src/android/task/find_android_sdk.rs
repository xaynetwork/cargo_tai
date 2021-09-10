use anyhow::anyhow;

use crate::{android::tools::AndroidSdk, task::Task, TaiResult};

use super::Context;

pub struct FindAndroidSdk;

impl Task for FindAndroidSdk {
    type Context = Context;

    fn run(&self, mut context: Self::Context) -> TaiResult<Self::Context> {
        let sdk = AndroidSdk::derive_sdk(
            context
                .requested
                .platform
                .android_ndk
                .as_ref()
                .ok_or_else(|| anyhow!("the option android_ndk is missing"))?,
        )?;

        context.android_sdk = Some(sdk);

        Ok(context)
    }
}
