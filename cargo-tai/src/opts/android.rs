use std::path::PathBuf;

use clap::Parser;
use tai_lib::common::opts;

#[derive(Parser, Debug)]
pub struct AndroidOptions {
    /// Android API level: only required when "target" is "*-linux-android*"
    ///
    /// You can find platform version information in Android Studio's Create New Project wizard.
    /// Example:
    ///
    /// `cargo-tai test --android-api-lvl 21`
    #[clap(
        long = "android-api-lvl",
        required_ifs(&[
            ("target", "x86_64-linux-android"),
            ("target", "aarch64-linux-android"),
            ("target", "i686-linux-android"),
            ("target", "armv7-linux-androideabi"),
        ])
    )]
    pub api_lvl: Option<u8>,

    /// The path to the android ndk: only required when "target" is "*-linux-android*"
    ///
    /// Example:
    ///
    /// `cargo-tai test --android-ndk ~/Library/Android/sdk/ndk/22.1.7171670`
    #[clap(
        long = "android-ndk",
        required_ifs(&[
            ("target", "x86_64-linux-android"),
            ("target", "aarch64-linux-android"),
            ("target", "i686-linux-android"),
            ("target", "armv7-linux-androideabi"),
        ]),
        env = "ANDROID_NDK_HOME"
    )]
    pub ndk: Option<PathBuf>,
}

impl From<AndroidOptions> for Option<opts::AndroidOptions> {
    fn from(AndroidOptions { api_lvl, ndk }: AndroidOptions) -> Self {
        match (api_lvl, ndk) {
            (Some(api_lvl), Some(ndk)) => Some(opts::AndroidOptions { api_lvl, ndk }),
            _ => None,
        }
    }
}
