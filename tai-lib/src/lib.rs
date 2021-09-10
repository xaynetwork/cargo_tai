use anyhow::Error;

pub mod android;
pub mod bundle;
pub mod command;
mod command_ext;
pub mod compiler;
#[cfg(feature = "ios")]
pub mod ios;
pub mod options;

type TaiResult<T> = Result<T, Error>;
