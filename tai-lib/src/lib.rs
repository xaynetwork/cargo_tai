use anyhow::Error;

pub mod android;
pub mod bundle;
pub mod command;
mod command_ext;
pub mod compiler;
#[cfg(feature = "ios")]
pub mod ios;
pub mod options;
pub mod project;
pub mod task;
pub mod tools;

type TaiResult<T> = Result<T, Error>;
