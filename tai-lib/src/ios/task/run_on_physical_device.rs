use std::path::Path;

use anyhow::bail;
use tracing::{info, instrument};

use crate::{
    common::{options::BinaryOptions, task::Task},
    ios::{platform::APP_ID, tools::ios_deploy},
    TaiResult,
};

use super::Context;

pub struct RunOnPhysicalDevice;

impl Task<Context> for RunOnPhysicalDevice {
    fn run(&self, context: Context) -> TaiResult<Context> {
        // if !mobile_provision
        //     .provisioned_devices
        //     .iter()
        //     .any(|d| d == device_id)
        // {
        //     bail!("device: {} not in provisioning profile", device_id);
        // }

        context
            .built_bundles()?
            .bundles
            .iter()
            .try_for_each(|bundle| install_and_launch(&bundle.root, context.binary()?))?;
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
