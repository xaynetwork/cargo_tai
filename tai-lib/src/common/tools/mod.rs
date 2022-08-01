use std::path::Path;

use cargo_metadata::Metadata;
use guppy::graph::PackageGraph;

use crate::TaiResult;
pub mod command_ext;
pub mod rsync;

pub use rsync::Rsync;

pub fn cargo_metadata<M: AsRef<Path>>(manifest: M) -> TaiResult<Metadata> {
    let mut cmd = cargo_metadata::MetadataCommand::new();
    let meta = cmd.manifest_path(manifest.as_ref()).exec()?;
    Ok(meta)
}

pub fn package_graph<M: AsRef<Path>>(manifest: M) -> TaiResult<PackageGraph> {
    let mut cmd = guppy::MetadataCommand::new();
    let meta = cmd.manifest_path(manifest.as_ref()).exec()?;
    let graph = meta.build_graph()?;

    Ok(graph)
}
