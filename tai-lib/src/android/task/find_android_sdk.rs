use crate::{android::tools::AndroidSdk, task::Task, TaiResult};

use super::Context;

pub struct FindAndroidSdk;

impl Task for FindAndroidSdk {
    type Context = Context;

    fn run(&self, mut context: Self::Context) -> TaiResult<Self::Context> {
        let sdk = AndroidSdk::derive_sdk(&context.requested.android_ndk)?;

        context.android_sdk = Some(sdk);

        Ok(context)
    }
}
