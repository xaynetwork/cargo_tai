use std::path::Path;

use cargo_metadata::{Metadata, MetadataCommand};

use crate::TaiResult;

pub fn cargo_metadata<M: AsRef<Path>>(manifest: M) -> TaiResult<Metadata> {
    let mut cmd = MetadataCommand::new();
    let meta = cmd.manifest_path(manifest.as_ref()).exec()?;
    Ok(meta)
}
