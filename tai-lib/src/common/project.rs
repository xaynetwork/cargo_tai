use std::{
    fs::{create_dir_all, remove_dir_all},
    path::PathBuf,
};

use crate::common::tools::cargo_metadata;
use cargo_metadata::Metadata;
use guppy::graph::PackageGraph;
use tracing::{debug, info};

use crate::TaiResult;

use super::tools::package_graph;

pub const CARGO_TAI_TARGET_DIR: &str = "cargo-tai";
pub const IOS_CACHE_DIR: &str = "cache-ios";
pub const RESOURCES_DIR: &str = "resources";

pub struct ProjectMetadata {
    pub meta: Metadata,
    pub cargo_opts: CargoOptions,
    pub tai_target: PathBuf,
    pub ios_cache: PathBuf,
    pub resources_dir: PathBuf,
    pub package_graph: PackageGraph,
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

        info!("Search for manifest file (`Cargo.toml`)");
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
        info!("Found manifest at  `{}`", manifest_path.display());

        Ok(Self {
            manifest_path,
            profile,
        })
    }
}

impl ProjectMetadata {
    pub fn from_cargo_args(cargo_args: &[String]) -> TaiResult<Self> {
        let cargo_opts = CargoOptions::from_cargo_args(cargo_args)?;
        info!(
            "Read metadata for package `{}`",
            cargo_opts.manifest_path.display()
        );
        let meta = cargo_metadata(&cargo_opts.manifest_path)?;
        info!("Create package graph");
        let package_graph = package_graph(&cargo_opts.manifest_path)?;

        let tai_target = meta
            .target_directory
            .join(CARGO_TAI_TARGET_DIR)
            .into_std_path_buf();
        debug!("`cargo-tai` target directory `{}`", tai_target.display());

        info!("Create iOS app cache");
        let ios_cache = tai_target.join(IOS_CACHE_DIR);
        create_dir_all(&ios_cache)?;
        debug!("iOS cache directory `{}`", ios_cache.display());

        info!("Create directory for resource metadata");
        let resources_dir = tai_target.join(RESOURCES_DIR);
        let _ = remove_dir_all(&resources_dir);
        create_dir_all(&resources_dir)?;
        debug!("Resource metadata directory `{}`", resources_dir.display());

        Ok(Self {
            meta,
            cargo_opts,
            tai_target,
            ios_cache,
            resources_dir,
            package_graph,
        })
    }
}
