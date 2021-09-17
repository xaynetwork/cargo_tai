use anyhow::bail;

use crate::{
    android::tools::adb::{self, Device},
    common::task::Task,
    TaiResult,
};

use super::Context;

pub struct ListDevices;

impl Task<Context> for ListDevices {
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let sdk = context.android_sdk()?;

        let devices = adb::devices(sdk)?
            .into_iter()
            .filter(|device| device.arch == context.opts.compiler.target.arch)
            .collect::<Vec<Device>>();

        if devices.is_empty() {
            bail!("no android device available")
        }

        context.devices = Some(devices);
        Ok(context)
    }
}
