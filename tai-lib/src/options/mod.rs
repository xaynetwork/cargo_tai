use std::path::PathBuf;

use cfg_expr::targets::TargetInfo;

use crate::command::Command;

#[derive(Debug, Clone)]
pub struct Options {
    pub general: GeneralOptions,

    pub platform: PlatformOptions,
}

#[derive(Debug, Clone)]
pub struct GeneralOptions {
    pub command: Command,
    pub compiler: CompilerOptions,
    pub binary: BinaryOptions,
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
    pub resources: Option<Vec<(String, PathBuf)>>,
}

#[derive(Debug, Clone)]
pub struct PlatformOptions {
    pub android_api_lvl: Option<u8>,
    pub android_ndk: Option<PathBuf>,
    pub ios_mobile_provision: Option<PathBuf>,
}
