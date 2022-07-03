# Cargo Tai

Test and benchmark your Rust library on mobile devices with ease.

Much of this project is based on the work of [cargo-dinghy](https://github.com/sonos/dinghy).
Furthermore, ideas were taken from the following projects:

- [cargo-ndk](https://github.com/bbqsrc/cargo-ndk)
- [polyhorn](https://github.com/polyhorn)
- [cargo-mobile](https://github.com/BrainiumLLC/cargo-mobile)
- [cargo-tarpaulin](https://github.com/xd009642/tarpaulin)
- [uikit-sys](https://github.com/simlay/uikit-sys)
- [android-ndk-rs](https://github.com/rust-windowing/android-ndk-rs)
- [cross](https://github.com/rust-embedded/cross)

## Features

- Run tests or benchmarks on iOS (simulator/real device) and Android (emulator/real device).
- Support for running benchmarks with `criterion` ([example](./test-project/benches/criterion.rs)).
- Bundle resource data that is required for your tests/benchmarks.

## Documentation

[Documentation](docs/README.md)

## Host Requirements

### iOS (macOS only)

- Xcode
- [ios-deploy](https://github.com/ios-control/ios-deploy)
- [libimobiledevice](https://libimobiledevice.org)
- rsync
- A valid iOS Development certificate
- rustup toolchains:
  - `x86_64-apple-ios`
  - `aarch64-apple-ios`

### Android

- Android SDK
- Android NDK
- [cargo-ndk](https://github.com/bbqsrc/cargo-ndk)
- rustup toolchains:
  - `x86_64-linux-android`
  - `aarch64-linux-android`
  - `i686-linux-android`
  - `armv7-linux-androideabi`

## Tested Configurations

**iOS**

- Real device: iPhone 8 with iOS 15.5 & iPhone 13 mini with iOS 15.5
- Simulator: iPhone 13 with iOS 15.5

**Android**

- Real device: Xiaomi Redmi Note 9 (aarch64), Android 10
- Emulator: x84_64, Android 10 (API 29)

**Host**

- Intel based Mac with macOS BigSur 12.4
- Xcode 13.4
- ios-deploy 1.11.4
- API level 21
- NDK 22.1.7171670
- cargo 1.62.0
- cargo toolchains:
  - `x86_64-linux-android`
  - `aarch64-linux-android`
  - `i686-linux-android`
  - `armv7-linux-androideabi`
  - `x86_64-apple-ios`
  - `aarch64-apple-ios`

## Limitations/Backwards Compatibility

- There are currently no plans to add support for older versions of Xcode and other tools.
- Resource data is included in all test/benchmark binary bundles even if they are not needed.
- support for `aarch64-apple-ios-sim` and `armv7-apple-ios` is currently not implemented
