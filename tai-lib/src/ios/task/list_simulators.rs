use anyhow::bail;

use crate::{common::task::Task, ios::tools::xcrun, TaiResult};

use super::Context;

pub struct ListSimulators;

impl Task<Context> for ListSimulators {
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let simulators = xcrun::list_booted_simulators()?;
        if simulators.is_empty() {
            bail!("no iOS simulator available")
        }

        context.simulators = Some(simulators);
        Ok(context)
    }
}
