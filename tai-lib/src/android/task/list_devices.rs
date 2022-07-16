use anyhow::bail;
use tracing::{info, instrument};

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
    #[instrument(name = "Find Device(s)", skip_all)]
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let env: &AndroidEnv = context.get();

        let devices = adb::devices(env)?
            .into_iter()
            .filter(|device| device.arch == context.get::<Options>().compiler.target.arch)
            .collect::<Vec<Device>>();
        if devices.is_empty() {
            bail!("No Android device available")
        }

        info!("Found the following Android device(s):");
        devices
            .iter()
            .for_each(|device| info!("ID: `{}`", device.id));

        context.insert(Devices(devices));
        Ok(context)
    }
}
