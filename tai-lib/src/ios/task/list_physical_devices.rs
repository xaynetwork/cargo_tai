use anyhow::bail;
use tracing::{info, instrument};

use crate::{common::task::Task, ios::tools::libimobiledevice, TaiResult};

use super::Context;

pub struct PhysicalDevices(pub Vec<libimobiledevice::Device>);

pub struct ListPhysicalDevices;

impl Task<Context> for ListPhysicalDevices {
    #[instrument(name = "Find Device(s)", skip(self, context))]
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let devices = libimobiledevice::list_devices()?;
        if devices.is_empty() {
            bail!("no iOS device available");
        }

        info!("Found the following iOS device(s):");
        devices
            .iter()
            .for_each(|device| info!("Name: `{}`, UDID: `{}`", device.name, device.id));

        context.insert(PhysicalDevices(devices));
        Ok(context)
    }
}
