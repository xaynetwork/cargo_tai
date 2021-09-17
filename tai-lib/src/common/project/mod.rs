use std::path::PathBuf;

use crate::common::tools::cargo_metadata;
use cargo_metadata::Metadata;

use crate::TaiResult;

pub const CARGO_TAI_TARGET_DIR: &str = "cargo-tai";
pub const IOS_NATIVE_TEST_WORKING_DIR: &str = "native-ios";
pub const IOS_CACHE_DIR: &str = "cache-ios";
pub const ANDROID_NATIVE_TEST_WORKING_DIR: &str = "native-android";

pub struct ProjectMetadata {
    pub meta: Metadata,
    pub cargo_opts: CargoOptions,
    pub tai_target: PathBuf,
    pub ios_cache: PathBuf,
    pub ios_working_dir: PathBuf,
    pub android_working_dir: PathBuf,
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

        let tai_target = meta
            .target_directory
            .join(CARGO_TAI_TARGET_DIR)
            .into_std_path_buf()
            .to_owned();

        let ios_working_dir = tai_target.join(IOS_NATIVE_TEST_WORKING_DIR);
        let android_working_dir = tai_target.join(ANDROID_NATIVE_TEST_WORKING_DIR);
        let ios_cache = tai_target.join(IOS_CACHE_DIR);

        Ok(Self {
            meta,
            cargo_opts,
            tai_target,
            ios_cache,
            ios_working_dir,
            android_working_dir,
        })
    }
}
