use std::{
    io::BufReader,
    process::{Command, Stdio},
};

use anyhow::{anyhow, bail};
use cargo_metadata::{camino::Utf8PathBuf, diagnostic::DiagnosticLevel, Artifact, Message};

use crate::{task::Options, TaiResult};

use super::{util::extend_with_options, BuildUnit};

fn is_test(artifact: Artifact) -> Option<Utf8PathBuf> {
    if let (Some(path), true) = (artifact.executable, artifact.profile.test) {
        Some(path)
    } else {
        None
    }
}

fn is_bench(artifact: Artifact) -> Option<Utf8PathBuf> {
    if let (Some(path), true) = (
        artifact.executable,
        artifact.target.kind.contains(&String::from("bench")),
    ) {
        Some(path)
    } else {
        None
    }
}

fn compile<F: Fn(Artifact) -> Option<Utf8PathBuf>>(
    mut cmd: Command,
    requested: &Options,
    f: F,
) -> TaiResult<Vec<BuildUnit>> {
    let cmd = extend_with_options(&mut cmd, requested)?;
    cmd.stdout(Stdio::piped());
    let mut child = cmd.spawn()?;
    let cargo_output = child
        .stdout
        .take()
        .ok_or(anyhow!("failed to read cargo output"))?;

    let reader = BufReader::new(cargo_output);
    let build_units: Result<Vec<BuildUnit>, _> = Message::parse_stream(reader)
        .into_iter()
        .try_fold(vec![], |mut acc, msg| match msg? {
            Message::CompilerArtifact(art) => {
                if let Some(path) = f(art) {
                    let unit = BuildUnit {
                        name: path
                            .file_name()
                            .ok_or(anyhow!("build artifact should have a name"))?
                            .to_string(),
                        executable: path.into(),
                        target: requested.target.clone(),
                    };
                    acc.push(unit);
                }
                Ok(acc)
            }
            Message::CompilerMessage(m) => match m.message.level {
                DiagnosticLevel::Error | DiagnosticLevel::Ice => {
                    bail!("{}", m);
                }
                _ => Ok(acc),
            },
            _ => Ok(acc),
        });

    child.wait()?;
    build_units
}

pub fn compile_tests(cmd: Command, requested: &Options) -> TaiResult<Vec<BuildUnit>> {
    compile(cmd, requested, is_test)
}

pub fn compile_bench(cmd: Command, requested: &Options) -> TaiResult<Vec<BuildUnit>> {
    compile(cmd, requested, is_bench)
}
