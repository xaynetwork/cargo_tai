use anyhow::anyhow;

use crate::{
    android::tools::AndroidSdk,
    common::{opts::Options, task::Task},
    TaiResult,
};

use super::Context;

pub struct FindAndroidSdk;

impl Task<Context> for FindAndroidSdk {
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let sdk = AndroidSdk::derive_sdk(
            &context
                .get::<Options>()
                .android
                .as_ref()
                .ok_or_else(|| anyhow!("no ndk"))?
                .ndk,
        )?;

        context.insert(sdk);

        Ok(context)
    }
}
