use anyhow::bail;

use crate::{
    android::tools::{
        adb::{self, Device},
        AndroidEnv,
    },
    common::{opts::Options, task::Task},
    TaiResult,
};

use super::Context;

pub struct Devices(pub Vec<Device>);

pub struct ListDevices;

impl Task<Context> for ListDevices {
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let env: &AndroidEnv = context.get();

        let devices = adb::devices(env)?
            .into_iter()
            .filter(|device| device.arch == context.get::<Options>().compiler.target.arch)
            .collect::<Vec<Device>>();

        if devices.is_empty() {
            bail!("no android device available")
        }

        context.insert(Devices(devices));
        Ok(context)
    }
}
