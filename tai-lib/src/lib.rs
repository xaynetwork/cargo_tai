use anyhow::Error;

pub mod android;
pub mod bundle;
mod command_ext;
pub mod compiler;
#[cfg(feature = "ios")]
pub mod ios;
pub mod task;

type TaiResult<T> = Result<T, Error>;
