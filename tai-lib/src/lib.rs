use anyhow::Error;

pub mod android;
pub mod common;
#[cfg(feature = "ios")]
pub mod ios;

type TaiResult<T> = Result<T, Error>;
