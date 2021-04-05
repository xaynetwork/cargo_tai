use anyhow::Error;

pub mod android;
pub mod bundle;
mod command_ext;
pub mod compiler;
pub mod task;

#[cfg(target_os = "macos")]
pub mod ios;

type TaiResult<T> = Result<T, Error>;
