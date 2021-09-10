use anyhow::bail;

use crate::{ios::tools::ios_deploy, task::Task, TaiResult};

use super::Context;

pub struct ListPhysicalDevices;

impl Task for ListPhysicalDevices {
    type Context = Context;

    fn run(&self, mut context: Self::Context) -> TaiResult<Self::Context> {
        let devices = ios_deploy::list_device()?;
        if devices.is_empty() {
            bail!("no iOS device available");
        }

        context.devices = Some(devices);
        Ok(context)
    }
}
