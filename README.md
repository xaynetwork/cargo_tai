# Cargo Tai

**Tested Configurations**

- intel based mac with macOS BigSur 11.2.3
- ios-deploy 1.11.4
- Xcode 12.4
- cargo 1.49.0
- iOS 14
- Android Studio 4.1.3
- toolchain:
  - "x86_64-linux-android"
  - "aarch64-linux-android"
  - "i686-linux-android"
  - "armv7-linux-androideabi"

## Requirements

### iOS (macOS only)

- Xcode
- [ios-deploy](https://github.com/ios-control/ios-deploy)
- a valid iOS Development certificate

### Android

1. Install [Android Studio](https://developer.android.com/studio)
2. Install Android NDK | [Guide](https://developer.android.com/studio/projects/install-ndk)
    - open Android Studio
    - go to `Configure` > `SDK Manager` > `Appearance & Behavior` > `System Settings` > `Android SDK` > `SDK Tools` and select `NDK (Side by side)` and `CMake`
    - click on `Apply` and wait for the installation to complete
    - close Android Studio

## Features

- run test or benchmark on iOS (simulator/real device) and Android (emulator/real device)
- support for `criterion` ([example](./test-project/benches/criterion.rs))
  - reports can be accessed via the file app or via iTunes
- include resources like test-data
- signing ios app

## Examples

```
// run benches in release mode
cargo-tai bench --target aarch64-apple-ios -- --release

// run tests on apple device
cargo-tai test --target aarch64-apple-ios

//
ios-deploy --bundle_id 'robertt.debug.com.Dinghy.Dinghy' --download --to .

// run tests on android device
ANDROID_NDK_HOME=~/Library/Android/sdk/ndk/22.1.7171670 cargo-tai test --target aarch64-linux-android --envs TAI=1 TAI2=22

//
cargo-tai test --target aarch64-apple-ios -r test_txt=./data/test.txt

//
cargo-tai test --target aarch64-apple-ios --args -Z,unstable-options,--report-time -- --release
```

## Inspired by

- [cargo-dinghy](https://github.com/sonos/dinghy)
- [cargo-ndk](https://github.com/bbqsrc/cargo-ndk)
- [polyhorn](https://github.com/polyhorn)
- [cargo-mobile](https://github.com/BrainiumLLC/cargo-mobile)
- [cargo-tarpaulin](https://github.com/xd009642/tarpaulin)
- [uikit-sys](https://github.com/simlay/uikit-sys)
- [android-ndk-rs](https://github.com/rust-windowing/android-ndk-rs)
- [cross](https://github.com/rust-embedded/cross)
