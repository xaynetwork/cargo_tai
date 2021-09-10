use std::path::Path;

use anyhow::bail;
use tracing::{info, instrument};

use crate::{
    ios::{platform::APP_ID, tools::ios_deploy},
    options::BinaryOptions,
    task::Task,
    TaiResult,
};

use super::Context;

pub struct RunOnPhysicalDevice;

impl Task for RunOnPhysicalDevice {
    type Context = Context;

    fn run(&self, context: Self::Context) -> TaiResult<Self::Context> {
        context
            .build_bundles()?
            .bundles
            .iter()
            .try_for_each(|bundle| {
                install_and_launch(&bundle.root, &context.requested.general.binary)
            })?;
        Ok(context)
    }
}

#[instrument(name = "install_launch", skip(bundle_root))]
fn install_and_launch<P>(bundle_root: P, binary_opt: &BinaryOptions) -> TaiResult<()>
where
    P: AsRef<Path>,
{
    match ios_deploy::launch_app(&bundle_root, &binary_opt.args, &binary_opt.envs) {
        Ok(_) => {
            info!("test result ok");
            Ok(())
        }
        Err(err) => {
            bail!(
                "test {} {} failed with: {}",
                APP_ID,
                &bundle_root.as_ref().display(),
                err
            )
        }
    }
}
