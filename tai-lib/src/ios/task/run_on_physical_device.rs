use std::path::Path;

use anyhow::bail;
use tracing::{info, instrument};

use crate::{
    common::{opts::BinaryOptions, task::Task},
    ios::{platform::APP_ID, tools::ios_deploy::IosDeployLaunch},
    TaiResult,
};

use super::Context;

pub struct RunOnPhysicalDevice;

impl Task<Context> for RunOnPhysicalDevice {
    #[instrument(name = "run_on_physical_device", skip(self, context))]
    fn run(&self, context: Context) -> TaiResult<Context> {
        let provisioned_devices = &context
            .signing_settings()?
            .mobile_provision
            .provisioned_devices;
        let bundles = context.built_bundles()?;
        let binary_opts = context.binary()?;

        context
            .devices()?
            .iter()
            .filter(|device| provisioned_devices.contains(&device.id))
            .try_for_each(|provisioned_device| {
                bundles.bundles.iter().try_for_each(|bundle| {
                    install_and_launch(
                        &provisioned_device.id,
                        &bundle.root,
                        binary_opts,
                        context.opts.cli.verbose,
                    )
                })
            })?;
        Ok(context)
    }
}

#[instrument(name = "install_launch", skip(bundle_root))]
fn install_and_launch<P>(
    device: &str,
    bundle_root: P,
    binary_opt: &BinaryOptions,
    verbose: bool,
) -> TaiResult<()>
where
    P: AsRef<Path>,
{
    let mut cmd = IosDeployLaunch::new(device, &bundle_root);
    cmd.non_interactive().no_wifi().debug();

    if let Some(ref args) = binary_opt.args {
        cmd.args(args);
    }
    if let Some(ref envs) = binary_opt.envs {
        cmd.envs(envs);
    }
    if verbose {
        cmd.verbose();
    }

    match cmd.execute() {
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
