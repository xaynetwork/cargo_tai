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

- Run tests or benchmarks on iOS (simulator/real device) and Android (emulator/real device)
- Support for running benchmarks with `criterion` ([example](./test-project/benches/criterion.rs))
- Bundle resource data that is required for your tests / benchmarks


## Documentation

[Documentation](docs/README.md)

## Host Requirements

### iOS (macOS only)

- Xcode
- [ios-deploy](https://github.com/ios-control/ios-deploy)
- A valid iOS Development certificate
- toolchains:
  - `x86_64-apple-ios`
  - `aarch64-apple-ios`

### Android

- [Android Studio](https://developer.android.com/studio)
- Android NDK
- toolchains:
  - `x86_64-linux-android`
  - `aarch64-linux-android`
  - `i686-linux-android`
  - `armv7-linux-androideabi`

## Tested Configurations

**iOS**
- Real device: iPhone 8 with iOS 14.5.1
- Simulator: iPhone 12 with iOS 14.5.1

**Android**
- Real device: Xiaomi Redmi Note 9 (aarch64), Android 10
- Emulator: x84_64, Android 10 (API 29)

**Host**
- Intel based Mac with macOS BigSur 11.3.1
- Xcode 12.5
- ios-deploy 1.11.4
- Android Studio 4.2
- API level 21
- NDK 22.1.7171670
- cargo 1.52.1
- cargo toolchains:
  - `x86_64-linux-android`
  - `aarch64-linux-android`
  - `i686-linux-android`
  - `armv7-linux-androideabi`
  - `x86_64-apple-ios`
  - `aarch64-apple-ios`

## Limitations/backward compatibility

- Tests and benchmarks can only be run on one device
  - (cargo-tai will use the first device that is returned by `adb devices -l`, `xcrun simctl list -j devices available booted` or `ios-deploy -c --json`)
- There are currently no plans to add support for older versions of Xcode and other tools
- Resource data is included in all test/benchmark binary bundles even if they are not needed
