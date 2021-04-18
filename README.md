# Cargo Tai

Test and benchmark your Rust library on mobile devices with ease.

**Note**

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
- Add resource data required for our tests / benchmarks

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


// run tests on android device
ANDROID_NDK_HOME=~/Library/Android/sdk/ndk/22.1.7171670 cargo-tai test --target aarch64-linux-android --envs TAI=1 TAI2=22


//
cargo-tai test --target aarch64-apple-ios --args -Z,unstable-options,--report-time -- --release
```

## Limitations/backward compatibility

- Tests and benchmarks can only be run on one device
  - (cargo-tai will use the first device that is returned by `adb devices -l`, `xcrun simctl list -j devices available booted` or `ios-deploy -c --json`)
- There are currently no plans to add support for older versions of Xcode and other tools
- Resource data is included in all test/benchmark binary bundles even if they are not needed

# Documentation


```
cargo-tai test --target aarch64-apple-ios -- --release
`-------’ `..’ `------------------------’    `.......’
 binary   mode       cargo-tai args          cargo args
```


## Setup iOS

![](assets/new_project.png)
![](assets/new_app.png)
![](assets/project_name.png)
![](assets/project_location.png)
![](assets/created_project.png)

```shell
security cms -D -i ~/Library/MobileDevice/Provisioning\ Profiles/<ID>.mobileprovision
```

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
...
  <key>Name</key>
	<string>iOS Team Provisioning Profile: your.domain.com.rust-lib</string>
...
</dict>
</plist>
```

## Runnings tests on iOS

We are using the `test-project` as an example

**Real device**

```shell
# run the tests and include the test data `test.txt`
cargo-tai test --target aarch64-apple-ios -r test_txt=./data/test.txt --ios-mobile-provision ~/Library/MobileDevice/Provisioning\ Profiles/<ID>.mobileprovision

# compile the tests in release mode
cargo-tai test --target aarch64-apple-ios --ios-mobile-provision ~/Library/MobileDevice/Provisioning\ Profiles/<ID>.mobileprovision -- --release
```

**Simulator**

```shell
# run the tests and include the test data `test.txt`
cargo-tai test --target x86_64-apple-ios

# pass additional arguments to the test binaries
cargo-tai test --target x86_64-apple-ios --args -Z,unstable-options,--report-time

# run a specific test in release mode
cargo-tai test --target x86_64-apple-ios --args test_x86_64_ios, -- --release
```

## Runnings benchmarks on iOS

We are using the `test-project` as an example

**Real device**

```shell
# run the benchmarks
cargo-tai bench --target aarch64-apple-ios --ios-mobile-provision ~/Library/MobileDevice/Provisioning\ Profiles/<ID>.mobileprovision -- --release

# download the /Documents folder
ios-deploy --bundle_id 'your.domain.com.rust-lib' --download=/Documents --to .

# open the report
open Documents/target/report/index.html
```

**Simulator**

```shell
# run the benchmarks
cargo-tai bench --target x86_64-apple-ios -- --release

# find the /Documents folder
xcrun simctl get_app_container booted cargo-tai data

# open the report
open /Users/xayn/Library/Developer/CoreSimulator/Devices/125E4403-E4AA-4AB0-ABC4-1E3C8882CD9F/data/Containers/Data/Application/32EB09BE-493A-456F-AC86-3EB9091129E2/Documents/target/report/index.html
```


via the `Files` App

![](assets/bench_data.png)

or via the `Finder`

![](assets/finder.png)

## Runnings tests on Android
