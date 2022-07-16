use anyhow::anyhow;
use tracing::{debug, instrument};

use crate::{
    android::tools::AndroidEnv,
    common::{opts::Options, task::Task},
    TaiResult,
};

use super::Context;

pub struct GetAndroidEnv;

impl Task<Context> for GetAndroidEnv {
    #[instrument(name = "Get Android Environment", skip(self, context))]
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let opts = context
            .get::<Options>()
            .android
            .as_ref()
            .ok_or_else(|| anyhow!("Failed to find Android NDK"))?;
        let env = AndroidEnv::derive_env(opts)?;

        debug!("{:#?}", env);

        context.insert(env);

        Ok(context)
    }
}
