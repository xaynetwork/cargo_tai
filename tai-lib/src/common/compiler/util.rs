use std::{
    io::BufReader,
    process::{Command, Stdio},
};

use anyhow::{anyhow, bail};
use cargo_metadata::{camino::Utf8PathBuf, diagnostic::DiagnosticLevel, Artifact, Message};

use crate::{common::opts::CompilerOptions, TaiResult};

use super::BuiltUnit;

pub fn is_test(artifact: Artifact) -> Option<Utf8PathBuf> {
    if let (Some(path), true) = (artifact.executable, artifact.profile.test) {
        Some(path)
    } else {
        None
    }
}

pub fn is_bench(artifact: Artifact) -> Option<Utf8PathBuf> {
    if let (Some(path), true) = (
        artifact.executable,
        artifact.target.kind.contains(&String::from("bench")),
    ) {
        Some(path)
    } else {
        None
    }
}

pub fn is_static_lib(mut artifact: Artifact) -> Option<Utf8PathBuf> {
    if let (Some(path), true) = (
        artifact.filenames.pop(),
        artifact.target.kind.contains(&String::from("staticlib")),
    ) {
        Some(path)
    } else {
        None
    }
}

pub fn compile<F: Fn(Artifact) -> Option<Utf8PathBuf>>(
    mut cmd: Command,
    requested: &CompilerOptions,
    f: F,
) -> TaiResult<Vec<BuiltUnit>> {
    let cmd = extend_with_cargo_args(&mut cmd, requested)?;
    cmd.stdout(Stdio::piped());
    let mut child = cmd.spawn()?;
    let cargo_output = child
        .stdout
        .take()
        .ok_or_else(|| anyhow!("failed to read cargo output"))?;

    let reader = BufReader::new(cargo_output);
    let built_units: Result<Vec<BuiltUnit>, _> = Message::parse_stream(reader)
        .into_iter()
        .try_fold(vec![], |mut acc, msg| match msg? {
            Message::CompilerArtifact(art) => {
                if let Some(path) = f(art) {
                    let unit = BuiltUnit {
                        name: path
                            .file_name()
                            .ok_or_else(|| anyhow!("build artifact should have a name"))?
                            .to_string(),
                        artifact: path.into(),
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
    built_units
}

pub fn extend_with_cargo_args<'a>(
    cmd: &'a mut Command,
    requested: &CompilerOptions,
) -> TaiResult<&'a mut Command> {
    cmd.args(&requested.cargo_args);

    cmd.args(&[
        "--target",
        requested.target.triple,
        "--message-format",
        "json",
    ]);

    Ok(cmd)
}
