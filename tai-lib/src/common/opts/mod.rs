use std::path::PathBuf;

use cfg_expr::targets::TargetInfo;

use super::command::Command;

#[derive(Debug, Clone)]
pub struct Options {
    pub command: Command,
    pub compiler: CompilerOptions,
    pub resources: Option<Vec<(String, PathBuf)>>,
    pub binary: Option<BinaryOptions>,
    pub build: Option<BuildOptions>,
    pub android: Option<AndroidOptions>,
    pub ios: Option<IosOptions>,
    pub cli: CliOptions,
}

#[derive(Debug, Clone)]
pub struct CompilerOptions {
    /// Build for the target triples
    pub target: TargetInfo<'static>,

    // cargo arguments
    pub cargo_args: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct BinaryOptions {
    pub args: Option<Vec<String>>,
    pub envs: Option<Vec<(String, String)>>,
}

#[derive(Debug, Clone)]
pub struct BuildOptions {
    pub template_dir: PathBuf,
    pub out_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct AndroidOptions {
    pub api_lvl: u8,
    pub ndk: PathBuf,
}

#[derive(Debug, Clone)]
pub struct IosOptions {
    pub mobile_provision: PathBuf,
}

#[derive(Debug, Clone)]
pub struct CliOptions {
    pub verbose: bool,
}
