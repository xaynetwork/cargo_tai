use std::process::Command;

use anyhow::anyhow;
use simctl::{list::DeviceState, Device, DeviceQuery, Simctl};

use crate::TaiResult;

const XCRUN: &'static str = "xcrun";

pub fn launch_app(
    dev_id: &str,
    app_id: &str,
    stdout: &str,
    args: &[&str],
    envs: &[&str],
) -> TaiResult<String> {
    let launch_output = Command::new(XCRUN)
        .args(&[
            "simctl",
            "launch",
            &format!("--stdout={}", stdout),
            "-w",
            dev_id,
            app_id,
        ])
        .args(args)
        .args(envs)
        .output()?;
    let launch_output = String::from_utf8_lossy(&launch_output.stdout);

    // Output from the launch command should be "APP_ID: $PID"
    Ok(launch_output
        .split_at(app_id.len() + 2)
        .1
        .trim()
        .to_string())
}

pub fn list_booted_simulators() -> TaiResult<Vec<Device>> {
    let simctl = Simctl::new();
    let devices = simctl.list().map_err(|err| anyhow!("{:?}", err))?;
    Ok(devices
        .devices()
        .into_iter()
        .available()
        .filter(|d| d.state == DeviceState::Booted)
        .cloned()
        .collect())
}
