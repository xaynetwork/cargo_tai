
# Documentation

<!--ts-->
* [Documentation](#documentation)
   * [Installation](#installation)
   * [Usage](#usage)
      * [iOS](#ios)
         * [Setup (real device only)](#setup-real-device-only)
         * [Running tests on iOS](#running-tests-on-ios)
         * [Running benchmarks on iOS](#running-benchmarks-on-ios)
      * [Android](#android)
         * [Runnings tests on Android](#runnings-tests-on-android)
         * [Running benchmarks on Android](#running-benchmarks-on-android)
   * [Cargo-tai logs](#cargo-tai-logs)

<!-- Added by: robert, at: Thu May 13 17:59:27 CEST 2021 -->

<!--te-->

## Installation

`cargo install --git`

## Usage

```
cargo-tai test --target aarch64-apple-ios -- --release
`-------’ `..’ `------------------------’    `.......’
 binary   mode       cargo-tai args          cargo args
```

Run `cargo-tai --help` for more information.

### iOS

#### Setup (real device only)

To be able to run tests or benchmarks on a real device we have to sign the app,
otherwise the installation will fail. `cargo-tai` expects a path to a valid
provisioning profile that it can use to sign the app. You can use an existing
profile or create a new one, as described in the following steps:

![](../assets/new_project.png)

First we open Xcode and create a new project.

![](../assets/new_app.png)

We choose `App` as a template and click `Next`.

![](../assets/project_name.png)

Next, we choose a product name (e.g. `rust-lib`), select a team and choose an unique org identifier.
Click `Next` to continue.

If you can't choose a team, you'll need to create one first via `Preferences` > `Accounts`> `+`.

![](../assets/project_location.png)

Choose a location for your project and click on `Create`.

![](../assets/created_project.png)

As the last step, we start the app on our device via Xcode. This step will install the certificate
that we have to accept via the phone settings `General` > `Device Management`.


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

#### Running tests on iOS

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

#### Running benchmarks on iOS

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

### Android

#### Runnings tests on Android

```shell
// run tests on android device
cargo-tai test --target aarch64-linux-android  --android-api-lvl 21 --android-ndk ~/Library/Android/sdk/ndk/22.1.7171670 -r test_txt=./data/test.txt
```

#### Running benchmarks on Android

## Cargo-tai logs

If you are interested in what `Cargo-Tai` does, you can increase the log verbosity via `RUST_LOG=debug`.
