# Cargo Tai


## Features

- Run tests or benchmarks on iOS (simulator/real device) and Android (emulator/real device).
- Support for `criterion` ([example](./test-project/benches/criterion.rs))
- Include resources data that are required by tour test/benchmarks

## Host Requirements

### iOS (macOS only)

- Xcode
- [ios-deploy](https://github.com/ios-control/ios-deploy)
- A valid iOS Development certificate

### Android

- [Android Studio](https://developer.android.com/studio)
- Android NDK


## Tested Configurations

**iOS**
- Real device: iPhone 8 with iOS 14
- Simulator: iPhone 12 with iOS 14

**Android**
- Real device: Xiaomi Redmi Note 9 (aarch64), Android 10
- Emulator: x84_64, Android 10 (API 29)

**Host**
- Intel based Mac with macOS BigSur 11.2.3
- Xcode 12.4
- ios-deploy 1.11.4
- Android Studio 4.1.3
- API level 21
- NDK 22.1.7171670
- cargo 1.49.0
- cargo toolchains:
  - `x86_64-linux-android`
  - `aarch64-linux-android`
  - `i686-linux-android`
  - `armv7-linux-androideabi`

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

This project is based on [cargo-dinghy](https://github.com/sonos/dinghy).
Furthermore ideas were taken from the following projects:

- [cargo-ndk](https://github.com/bbqsrc/cargo-ndk)
- [polyhorn](https://github.com/polyhorn)
- [cargo-mobile](https://github.com/BrainiumLLC/cargo-mobile)
- [cargo-tarpaulin](https://github.com/xd009642/tarpaulin)
- [uikit-sys](https://github.com/simlay/uikit-sys)
- [android-ndk-rs](https://github.com/rust-windowing/android-ndk-rs)
- [cross](https://github.com/rust-embedded/cross)

## Limitations/backward compatibility

- currently only one device is supported
- you can not specify the device
- currently there are no plans to add support for older versions of xcode, ...
- test data does not allow intra-links 

# Documentation


```
cargo-tai test --target aarch64-apple-ios -- --release
`-------’ `..’ `------------------------’    `.......’
 binary   mode       cargo-tai args          cargo args
```

## Runnings tests on iOS

We are using the `test-project` project as an example

```
// run the tests and include the test data `test_txt`
cargo-tai test --target aarch64-apple-ios -r test_txt=./data/test.txt

// pass additional arguments to cargo
cargo-tai test --target aarch64-apple-ios -- --release

// pass additional arguments to cargo
cargo-tai test --target aarch64-apple-ios -- --release
```

## Runnings benchmarks on iOS

We are using the `test-project` project as an example

```
// run the benchmarks
cargo-tai bech --target aarch64-apple-ios

ios-deploy --bundle_id 'robertt.debug.com.Dinghy.Dinghy' --download --to .
```


## Runnings tests on Android
