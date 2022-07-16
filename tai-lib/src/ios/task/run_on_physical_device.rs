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
    #[instrument(name = "Run On Device(s)", skip_all)]
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

                    info!(
                        "On `{}` run bundle `{}`",
                        provisioned_device.id, bundle.build_unit.name
                    );
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

fn install_and_launch<B: AsRef<Path>, A: AsRef<Path>>(
    device: &str,
    bundle_root: B,
    app_deltas: A,
    binary_opt: &BinaryOptions,
    verbose: bool,
) -> TaiResult<()> {
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
            info!("Run completed successfully!");
            Ok(())
        }
        Err(err) => {
            bail!(
                "Run `{}` `{}` failed with error: {}",
                APP_ID,
                &bundle_root.as_ref().display(),
                err
            )
        }
    }
}
