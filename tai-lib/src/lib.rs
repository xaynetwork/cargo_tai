use anyhow::Error;

mod command_ext;
pub mod task;

#[cfg(target_os = "macos")]
pub mod ios;

type TaiResult<T> = Result<T, Error>;
