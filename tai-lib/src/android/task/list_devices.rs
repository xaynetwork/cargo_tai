use anyhow::{anyhow, bail};

use crate::{
    android::tools::adb::{self, Device},
    task::Task,
    TaiResult,
};

use super::Context;

pub struct ListDevices;

impl Task for ListDevices {
    type Context = Context;

    fn run(&self, mut context: Self::Context) -> TaiResult<Self::Context> {
        let sdk = context
            .android_sdk
            .as_ref()
            .ok_or_else(|| anyhow!("no android sdk"))?;

        let devices = adb::devices(sdk)?
            .into_iter()
            .filter(|device| device.arch == context.requested.general.compiler.target.arch)
            .collect::<Vec<Device>>();

        if devices.is_empty() {
            bail!("no android device available")
        }

        context.devices = Some(devices);
        Ok(context)
    }
}
