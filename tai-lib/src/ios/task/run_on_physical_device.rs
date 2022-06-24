use std::{fs::create_dir_all, path::Path};

use anyhow::bail;
use tracing::{info, instrument};

use crate::{
    common::{
        opts::{BinaryOptions, Options},
        project::ProjectMetadata,
        task::Task,
        tools::Rsync,
    },
    ios::{
        bundle::{bundler::APP_DISPLAY_NAME, signing::SigningSettings},
        platform::APP_ID,
        tools::ios_deploy::IosDeployLaunch,
    },
    TaiResult,
};

use super::{
    create_signed_bundles::SignedBuiltBundles,
    list_physical_devices::PhysicalDevices,
    Context,
};

pub struct RunOnPhysicalDevice;

impl Task<Context> for RunOnPhysicalDevice {
    #[instrument(name = "run_on_physical_device", skip(self, context))]
    fn run(&self, context: Context) -> TaiResult<Context> {
        let provisioned_devices = &context
            .get::<SigningSettings>()
            .mobile_provision
            .provisioned_devices;
        let bundles = &context.get::<SignedBuiltBundles>().0;
        let ios_cache = &context.get::<ProjectMetadata>().ios_cache;
        let opts: &Options = context.get();
        let default = BinaryOptions::default();
        let binary_opts = match opts.binary.as_ref() {
            Some(opts) => opts,
            None => &default,
        };

        let app_deltas = ios_cache.join("app_deltas");
        create_dir_all(&app_deltas)?;

        context
            .get::<PhysicalDevices>()
            .0
            .iter()
            .filter(|device| provisioned_devices.contains(&device.id))
            .try_for_each(|provisioned_device| {
                bundles.bundles.iter().try_for_each(|bundle| {
                    let mut cmd = Rsync::new(&bundle.root, &ios_cache);
                    cmd.archive().delete();
                    if opts.cli.verbose {
                        cmd.verbose();
                    }
                    cmd.execute()?;

                    install_and_launch(
                        &provisioned_device.id,
                        ios_cache.join(format!("{}.app", APP_DISPLAY_NAME)),
                        &app_deltas,
                        binary_opts,
                        opts.cli.verbose,
                    )
                })
            })?;
        Ok(context)
    }
}

#[instrument(name = "install_launch", skip(bundle_root, app_deltas))]
fn install_and_launch<P1, P2>(
    device: &str,
    bundle_root: P1,
    app_deltas: P2,
    binary_opt: &BinaryOptions,
    verbose: bool,
) -> TaiResult<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let mut cmd = IosDeployLaunch::new(device, &bundle_root);
    cmd.non_interactive()
        .no_wifi()
        .debug()
        .app_deltas(app_deltas);

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
