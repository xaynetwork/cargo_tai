use anyhow::anyhow;

use crate::{
    android::tools::AndroidEnv,
    common::{opts::Options, task::Task},
    TaiResult,
};

use super::Context;

pub struct GetAndroidEnv;

impl Task<Context> for GetAndroidEnv {
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let opts = context
            .get::<Options>()
            .android
            .as_ref()
            .ok_or_else(|| anyhow!("no ndk"))?;
        let env = AndroidEnv::derive_env(opts)?;

        context.insert(env);

        Ok(context)
    }
}
