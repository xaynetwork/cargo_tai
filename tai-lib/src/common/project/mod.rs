use std::path::PathBuf;

use crate::common::tools::cargo_metadata;
use cargo_metadata::Metadata;

use crate::TaiResult;

pub const CARGO_TAI_TARGET_DIR: &str = "cargo-tai";
pub const IOS_DIR: &str = "native-ios";
pub const ANDROID_DIR: &str = "native-android";

pub struct ProjectMetadata {
    pub meta: Metadata,
    pub cargo_opts: CargoOptions,
}

pub struct CargoOptions {
    pub manifest_path: PathBuf,
    pub profile: Profile,
}

#[derive(Debug, Clone, Copy)]
pub enum Profile {
    Debug,
    Release,
}

impl Profile {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Debug => "debug",
            Self::Release => "release",
        }
    }
}

impl CargoOptions {
    pub fn from_cargo_args(args: &[String]) -> TaiResult<Self> {
        let profile = match args.iter().any(|arg| arg == "--release") {
            false => Profile::Debug,
            true => Profile::Release,
        };

        // https://docs.rs/cargo_metadata/0.14.0/cargo_metadata/#examples
        let mut iter = args
            .iter()
            .skip_while(|val| !val.starts_with("--manifest-path"));
        let manifest_path = match iter.next() {
            Some(p) if p == "--manifest-path" => iter
                .next()
                .ok_or_else(|| anyhow::anyhow!("no manifest"))?
                .into(),
            Some(p) => p.trim_start_matches("--manifest-path=").into(),
            None => std::env::current_dir()?.join("Cargo.toml"),
        };

        Ok(Self {
            manifest_path,
            profile,
        })
    }
}

impl ProjectMetadata {
    pub fn from_cargo_args(cargo_args: &[String]) -> TaiResult<Self> {
        let cargo_opts = CargoOptions::from_cargo_args(cargo_args)?;
        let meta = cargo_metadata(&cargo_opts.manifest_path)?;

        Ok(Self { meta, cargo_opts })
    }

    pub fn tai_target_dir(&self) -> PathBuf {
        let project_target_dir = &self.meta.target_directory;
        project_target_dir
            .join(CARGO_TAI_TARGET_DIR)
            .into_std_path_buf()
    }

    pub fn ios_dir(&self) -> PathBuf {
        let project_target_dir = &self.meta.target_directory;
        project_target_dir
            .join(CARGO_TAI_TARGET_DIR)
            .join(IOS_DIR)
            .into_std_path_buf()
    }

    pub fn android_dir(&self) -> PathBuf {
        let project_target_dir = &self.meta.target_directory;
        project_target_dir
            .join(CARGO_TAI_TARGET_DIR)
            .join(ANDROID_DIR)
            .into_std_path_buf()
    }
}
