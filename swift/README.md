# Khíín for iOS & macOS

Folders:

- `Khiin`: Currently a blank app with a text field simply for testing the IME
- `Keyboard`: The iOS Keyboard Extension (`.appex`) Bundle code
- `KhiinIM`: The macOS Input Method
- `Shared`: Shared code between the iOS and macOS apps
- `bridge`: A bridge module for Swift-Rust communication, using `swift-bridge`
- `Protos`: The generated `.pb.swift` protobuf glass files

## XCode Notes

The XCode project must be built with `xcodegen`, it does not get checked in to
the repo. See instructions below.

When running the iOS simulator, XCode can be very flaky with respect to
rebuilding and running the latest code. If you need debugging (breakpoints and
logging), you must have your Scheme set to run the `Keyboard` target, not the
`Khiin` target. However, since the `Khiin` target contains the `Keyboard` target
as an embedded `.appex`, I have found the best results by following these steps:

1. Work on iOS code in the `Keyboard` folder
2. Build the `Khiin` target and run it on the simulator
3. Stop the execution of this process (hit the stop button at the top left)
4. Build the `Keyboard` target, and select the `Khiin` app to launch

This seems to work most of the time.

You will also need to enable the IME from the app settings to see it in the
keyboard selection menu. Go to Settings > Khiin > Keyboards > Khiin PJH to
enable it. Note that when selecting the keyboard, it is a bit flaky and does not
always load properly the first time, you may have to select it twice. (I am told
this is not a problem on actual devices, but I do not have one for testing.)

## iOS Notes

- The `Keyboard.appex` bundle will be embedded in the `Khiin.app` bundle for
  delivery onto the device.
- The iOS bundle identifier must start with something like `com.` or `org.`, and
  maybe a few others. However, we cannot use arbitrary bundle identifiers, as
  the original attempt to use `be.chiahpa` resulted in the IME not being
  recognized by the system.
- The `Khiin` package (the app) must have a `Settings.bundle` folder in order
  for the Keyboard extension to show up in system settings on iOS. The settings
  bundle does not need to have any properties or settings.

## macOS Notes

- The macOS Input Method bundle identifier **must** have `.inputmethod.` in the
  **3rd** position. That is: `a.b.inputmethod.c` is a valid bundle identifier.
  This is apparently not documented anywhere, but it was mentioned on some
  obscure websites and in old header files.
- The KhiinIM build script (`cargo make build-khiinim`) will place the
  `KhiinIM.app` bundle into `~/Library/Input Methods/`. OSX reads from this
  folder to load input methods. The first time you build the IM, you need to log
  out and log back in to refresh the available input methods. Subsequent builds
  should "just work", but YMMV.

---

## Development Environment

### Quickstart

Just run this:

```bash
brew install swift-protobuf xcodegen
rustup target add aarch64-apple-darwin x86_64-apple-darwin aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
cargo install --force swift-bridge-cli
cargo install --force cargo-make
cargo make
```

This will build all prereqs and the macOS input method. From here:

- The iOS app can be built in Xcode using the simulator directly.
- To rebuild the macOS IM after making changes, run: `cargo make build-khiinim`

### Details

1. Use `cargo-make` to run all of the build steps.

```bash
cargo install --force cargo-make
```

2. Install the appropriate toolchains:

```bash
# For macOS
rustup target add aarch64-apple-darwin x86_64-apple-darwin

# For iOS
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios

# For iOS simulator
rustup target add aarch64-apple-ios-sim x86_64-apple-ios
```

3. Files in `Protos` are generated using Apple's `swift-protobuf` plugin.

```bash
brew install swift-protobuf
```

Once everything is done, you can run `cargo make` to prepare everything for
building the mac/iOS apps. The first time takes a little longer to build the
libraries, but after that you should see an output similar to the following:

```
$ cargo make
[cargo-make] INFO - cargo make 0.36.6
[cargo-make] INFO - Calling cargo metadata to extract project info
[cargo-make] INFO - Cargo metadata done
[cargo-make] INFO - Build File: Makefile.toml
[cargo-make] INFO - Task: default
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Execute Command: "sh" "./swift/bridge/build.sh"
    Blocking waiting for file lock on build directory
   Compiling khiin_swift v0.1.0 (...)
    Finished dev [unoptimized + debuginfo] target(s) in 1.21s
    // ...snip...
[cargo-make] INFO - Execute Command: "swift-bridge-cli" "create-package" // ...snip...
[cargo-make] INFO - Running Task: build-mac-protos
[cargo-make] INFO - Execute Command: "python3" "src/sql_gen.py" // ...snip...
Building database, please wait...Output written to out/khiin_db.sql:
 - 12242 inputs ("frequency" table)
 - 25403 tokens ("conversions" table)
 - 1514 syllables ("syllables" table)
[cargo-make] INFO - Running Task: db-copy
[cargo-make] INFO - Running Task: db-copy-mac
[cargo-make] INFO - Running Task: db-copy-to-target
[cargo-make] INFO - Execute Command: "cargo" "build" "--manifest-path=cli/Cargo.toml"
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
[cargo-make] INFO - Build Done in 10.44 seconds.
```

### Bridge Module

`cargo make build-swift-bridge`:

The `bridge` module is compiled using `swift-bridge` into a `KhiinBridge` swift
package that can be used in XCode directly. Building is a two-step process.

1. Use the `build.sh` (which uses `build.rs`) to produce the `libkhiin_swift.a`
   binaries for each platform.
2. Use the `swift-bridge-cli` tool to package the generated code and the
   binaries into a Swift Package for use in XCode.

The package is generated to `swift/KhiinBridge`, and can be added to XCode as a
normal local package dependency. The targets using this package must add a Build
Phase for the package.

- Right click the project, click `Add Packages...` -> `Add Local..` and select
  the `KhiinBridge` folder
- Navigate to the target -> `Build Phases` -> `Target Dependencies` and click
  the `+` button, selecting `KhiinBridge
- Repeat for `Link Binary With Libraries`

This configuration should already be done, so you shouldn't need to change it.

The binaries produced by `bridge/build.sh` are:

- `target/universal-ios/(debug|release)/libkhiin_swift.a` - for the simulator
- `target/aarch64-apple-ios/(debug|release)/libkhiin_swift.a` - for iOS devices
- `target/universal-macos/(debug|release)/libkhiin_swift.a` - for macOS devices

### KhiinIM (macOS)

`cargo make build-khiinim`

The first time you run this command, you will need to log out and log back in to
activate the IME. If you leave Xcode open while running this command, Xcode may
get very flaky and break the build process at any time, for unknown reasons. To
fix this is very simple: close Xcode and re-build everything:

```bash
cargo make clean && cargo make
```
