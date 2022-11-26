use anyhow::bail;
use tracing::instrument;

use crate::{common::task::Task, ios::tools::libimobiledevice, TaiResult};

use super::Context;

pub struct PhysicalDevices(pub Vec<libimobiledevice::Device>);

pub struct ListPhysicalDevices;

impl Task<Context> for ListPhysicalDevices {
    #[instrument(name = "list_physical_devices", skip(self, context))]
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let devices = libimobiledevice::list_devices()?;
        if devices.is_empty() {
            bail!("no iOS device available");
        }

        context.insert(PhysicalDevices(devices));
        Ok(context)
    }
}
