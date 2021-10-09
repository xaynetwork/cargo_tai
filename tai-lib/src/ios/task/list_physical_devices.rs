use anyhow::bail;
use tracing::instrument;

use crate::{common::task::Task, ios::tools::ios_deploy, TaiResult};

use super::Context;

pub struct ListPhysicalDevices;

impl Task<Context> for ListPhysicalDevices {
    #[instrument(name = "list_physical_devices", skip(self, context))]
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let devices = ios_deploy::list_device()?;
        if devices.is_empty() {
            bail!("no iOS device available");
        }

        context.devices = Some(devices);
        Ok(context)
    }
}
