use crate::{android::tools::AndroidSdk, common::task::Task, TaiResult};

use super::Context;

pub struct FindAndroidSdk;

impl Task<Context> for FindAndroidSdk {
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let sdk = AndroidSdk::derive_sdk(&context.requested.android_ndk)?;

        context.android_sdk = Some(sdk);

        Ok(context)
    }
}
