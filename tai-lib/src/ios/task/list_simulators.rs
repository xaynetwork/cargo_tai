use anyhow::bail;

use crate::{ios::tools::xcrun, task::Task, TaiResult};

use super::Context;

pub struct ListSimulators;

impl Task for ListSimulators {
    type Context = Context;

    fn run(&self, mut context: Self::Context) -> TaiResult<Self::Context> {
        let simulators = xcrun::list_booted_simulators()?;
        if simulators.is_empty() {
            bail!("no iOS simulator available")
        }

        context.simulators = Some(simulators);
        Ok(context)
    }
}
