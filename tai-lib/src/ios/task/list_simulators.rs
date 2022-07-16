use anyhow::bail;
use tracing::instrument;

use crate::{common::task::Task, ios::tools::xcrun, TaiResult};

use super::Context;
pub struct Simulators(pub Vec<simctl::Device>);

pub struct ListSimulators;

impl Task<Context> for ListSimulators {
    #[instrument(name = "list_simulators", skip(self, context))]
    fn run(&self, mut context: Context) -> TaiResult<Context> {
        let simulators = xcrun::list_booted_simulators()?;
        if simulators.is_empty() {
            bail!("no iOS simulator available")
        }

        context.insert(Simulators(simulators));
        Ok(context)
    }
}
