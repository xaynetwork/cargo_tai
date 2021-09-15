use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, Error};
use simctl::{get_app_container::Container, Device};
use tempfile::TempDir;
use tracing::{debug, info, instrument};

use crate::{
    common::{options::BinaryOptions, task::Task},
    ios::{
        platform::APP_ID,
        tools::{lldb, xcrun},
    },
    TaiResult,
};

use super::Context;

pub struct RunOnSimulators;

impl Task<Context> for RunOnSimulators {
    fn run(&self, context: Context) -> TaiResult<Context> {
        let bundles = context.built_bundles()?;

        context.simulators()?.iter().try_for_each(|simulator| {
            bundles.bundles.iter().try_for_each(|bundle| {
                install_and_launch(simulator, &bundle.root, context.binary()?)
            })
        })?;
        Ok(context)
    }
}

#[instrument(name = "install_launch", fields(device = %device.udid), skip(bundle_root))]
fn install_and_launch<P: AsRef<Path>>(
    device: &Device,
    bundle_root: P,
    binary_opt: &BinaryOptions,
) -> TaiResult<()> {
    let bundle_root = bundle_root.as_ref();
    info!("uninstall app with app id: {}", APP_ID);
    device
        .uninstall(APP_ID)
        .map_err(|_| anyhow!("failed to uninstall: {}", APP_ID))?;

    info!("install: {}", bundle_root.display());
    device
        .install(bundle_root.as_ref())
        .map_err(|_| anyhow!("failed to install: {}", APP_ID))?;

    info!("launch app with app id:: {}", APP_ID);
    match launch_app(device, binary_opt)? {
        0 => {
            info!("test result ok");
            Ok(())
        }
        ec => {
            bail!(
                "test {} {} failed with exit code: {}",
                APP_ID,
                bundle_root.display(),
                ec
            )
        }
    }
}

fn launch_app(device: &Device, binary_opt: &BinaryOptions) -> TaiResult<u32> {
    let install_path = device
        .get_app_container(APP_ID, &Container::App)
        .map_err(|err| anyhow!("{:?}", err))?;
    let stdout = install_path.join("stdout");
    let stdout_str = stdout.to_string_lossy();
    debug!("write stdout to: {}", stdout_str);

    let app_pid = xcrun::launch_app(
        &device.udid,
        APP_ID,
        &stdout_str,
        &binary_opt.args,
        &binary_opt.envs,
    )?;
    debug!("app pid: {}", app_pid);
    let (lldb_path, guard) = create_lldb_script(&app_pid)?;
    let output = lldb::run_source(&lldb_path)?;

    let stdout_file = File::open(stdout)?;
    let mut reader = BufReader::new(stdout_file);
    std::io::stdout().write_all(reader.fill_buf()?)?;

    guard.close()?; // delete lldb script
    extract_lldb_exit_status(&output.stdout)
}

fn create_lldb_script(app_pid: &str) -> Result<(PathBuf, TempDir), Error> {
    // Attaching to the processes needs to be done in a script, not a
    // commandline parameter or lldb will say "no simulators found".
    let temp_dir = tempfile::Builder::new().prefix(app_pid).tempdir()?;
    let path = temp_dir.path().join("lldb-script");

    let mut file = File::create(&path)?;
    file.write_fmt(format_args!(
        include_str!("../templates/lldb.tmpl"),
        app_pid = app_pid,
    ))?;

    debug!("temp lldb-script: {}", path.display());
    Ok((path, temp_dir))
}

fn extract_lldb_exit_status(stdout: &[u8]) -> TaiResult<u32> {
    let output = String::from_utf8_lossy(stdout).to_string();
    debug!("LLDB output:\n{}", output);
    /*
    The stdout from lldb is something like:

    (lldb) attach 34163
    Process 34163 stopped
    * thread #1, stop reason = signal SIGSTOP
        frame #0: 0x00000001019cd000 dyld`_dyld_start
    dyld`_dyld_start:
    ->  0x1019cd000 <+0>: popq   %rdi
        0x1019cd001 <+1>: pushq  x0
        0x1019cd003 <+3>: movq   %rsp, %rbp
        0x1019cd006 <+6>: andq   $-0x10, %rsp
    Target 0: (Dinghy) stopped.

    Executable module set to .....
    Architecture set to: x86_64h-apple-ios-.
    (lldb) continue
    Process 34163 resuming
    Process 34163 exited with status = 101 (0x00000065)

    (lldb) quit

    We need the "exit with status" line which is the 2rd from the last
    */
    let exit_status_line: Option<&str> = output.lines().rev().nth(2);
    let tokens: Vec<&str> = if let Some(exit_status_line) = exit_status_line {
        exit_status_line.split_whitespace().rev().collect()
    } else {
        bail!(
            "failed to get the exit status line from lldb output: {:?}",
            exit_status_line
        );
    };

    if let Some(exit_status) = tokens.get(1) {
        exit_status
            .parse::<u32>()
            .map_err(|err| anyhow!("failed to determine exit status: {}", err))
    } else {
        bail!(
            "failed to parse lldb exit line for an exit status. {:?}",
            tokens
        )
    }
}
