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

    /// The path to the android sdk.
    ///
    /// Example:
    ///
    /// `cargo-tai test --android-sdk ~/Library/Android/sdk`
    #[clap(long = "android-sdk", env = "ANDROID_SDK_HOME")]
    pub sdk: Option<PathBuf>,

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

    /// A comma-separated list of arguments to pass to cargo ndk.
    ///
    /// Example:
    ///
    /// `cargo-tai test --cargo-ndk-args --no-strip,--bindgen`
    #[clap(short, long, allow_hyphen_values = true, use_delimiter = true)]
    pub cargo_ndk_args: Option<Vec<String>>,
}

impl From<AndroidOptions> for Option<opts::AndroidOptions> {
    fn from(
        AndroidOptions {
            api_lvl,
            sdk,
            ndk,
            cargo_ndk_args,
        }: AndroidOptions,
    ) -> Self {
        match (api_lvl, ndk) {
            (Some(api_lvl), Some(ndk)) => Some(opts::AndroidOptions {
                api_lvl,
                sdk,
                ndk,
                cargo_ndk_args,
            }),
            _ => None,
        }
    }
}
