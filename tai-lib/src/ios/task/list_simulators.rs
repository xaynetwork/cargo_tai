use anyhow::bail;
use tracing::{info, instrument};

use crate::{common::task::Task, ios::tools::xcrun, TaiResult};

use super::Context;
pub struct Simulators(pub Vec<simctl::Device>);

pub struct ListSimulators;

impl Task<Context> for ListSimulators {
    #[instrument(name = "Find Simulator(s)", skip(self, context))]
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let simulators = xcrun::list_booted_simulators()?;
        if simulators.is_empty() {
            bail!("No iOS simulator available")
        }

        info!("Found the following iOS simulator(s):");
        simulators.iter().for_each(|device| {
            info!(
                "Name: `{}`, UDID: `{}`",
                device.info().name,
                device.info().udid
            )
        });

        context.insert(Simulators(simulators));
        Ok(context)
    }
}
