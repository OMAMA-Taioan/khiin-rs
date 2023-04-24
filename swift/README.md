# Khiin iOS & macOS

Folders:

- `Khiin`: Currently a blank app with a text field simply for testing the IME
- `EngineBindings`: A library containing the rust binding code and a Swift
  wrapper class for using it in the apps
- `Keyboard`: The iOS Keyboard Extension (`.appex`) Bundle code

The `Keyboard.appex` bundle will be embedded in the `Khiin.app` bundle for
delivery onto the device. 

## XCode

When running the iOS simulator, XCode can be very flaky with respect to
rebuilding and running the latest code. If you need debugging (breakpoints and
logging), you must have your Scheme set to run the `Keyboard` target, not the
`Khiin` target. However, since the `Khiin` target contains the `Keyboard`
target, I have found the best results by following these steps:

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

Note that you maye also see a warning when building the project:

```
Linking against a dylib which is not safe for use in application extensions:
/Users/.../SwiftProtobuf...
```

I have not yet figured out how to fix this warning, although it does not seem to
prevent any problems and the app runs as expected, and sometimes does not show
up at all.

## Development Environment

### Quickstart

Just run this:

```bash
brew install swift-protobuf
rustup target add aarch64-apple-darwin x86_64-apple-darwin aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
cargo install --force cargo-make
cargo install --force cbindgen
cargo make
```

You are now ready to build the iOS app 😄

### Details

1. Use `cargo-make` for the commands listed in this document:

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

3. Files in the `EngineBindings/generated` file are automatically generated as
explained in this README. If you are not modifying the rust or protobuf code,
you should not need to modify these files. If you do intend to make
modifications, then you must install these dependencies to regenerate the files:

- `cbindgen`: a tool for creating the C header files used from Swift
- `swift-protobuf`: Apple's Protobuf extension for Swift

```bash
cargo install --force cbindgen
brew install swift-protobuf
```

Once everything is done, you can run `cargo make` to prepare everything for
building the mac/iOS apps. You should see an output similar to the following:

```
$ cargo make
[cargo-make] INFO - cargo make 0.36.6
[cargo-make] INFO - Calling cargo metadata to extract project info
[cargo-make] INFO - Cargo metadata done
[cargo-make] INFO - Build File: Makefile.toml
[cargo-make] INFO - Task: default
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Execute Command: "sh" "./swift/EngineBindings/build.sh"
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
[cargo-make] INFO - Running Task: mac-builddirs
[cargo-make] INFO - Execute Command: "cbindgen" "--config" // ...snip...
[cargo-make] INFO - Running Task: build-mac-protos
[cargo-make] INFO - Execute Command: "python3" "src/sql_gen.py" // ...snip...
Building database, please wait...Output written to out/khiin_db.sql:
    // ...snip...
[cargo-make] INFO - Running Task: db-copy
[cargo-make] INFO - Running Task: db-copy-mac
[cargo-make] INFO - Running Task: db-copy-to-target
[cargo-make] INFO - Execute Command: "cargo" "build" "--manifest-path=cli/Cargo.toml"
    // ...snip...
   Compiling khiin_protos v0.1.0 (/Users/ed/aiongg/khiin-rs/protos)
   Compiling khiin v0.1.0 (/Users/ed/aiongg/khiin-rs/khiin)
   Compiling khiin_cli v0.1.0 (/Users/ed/aiongg/khiin-rs/cli)
    Finished dev [unoptimized + debuginfo] target(s) in 5.50s
[cargo-make] INFO - Build Done in 11.15 seconds.
```

### XCode linker configuration

This configuration should already be done, so you shouldn't need to change it
unless you are editing the builds.

In order to link to the `EngineBindings` target, other targets must add some
linker flags:

- Khiin Project > Target > Build Settings > Linking > Other Linker Flags

The debug and release flags should use the debug and release `khiin_swift.a`
library files as appropriate:

Debug:
- -lkhiin_swift
- -L$(PROJECT_DIR)/../target/universal-ios/debug (for simulator)
- -L$(PROJECT_DIR)/../target/aarch64-apple-ios/debug (for iOS)

Release:
- -lkhiin_swift
- -L$(PROJECT_DIR)/../target/universal-ios/release (for simulator)
- -L$(PROJECT_DIR)/../target/aarch64-apple-ios/release (for iOS)


Also under the Build Settings > Swift Compiler - General section, you must set
the bridging header:

- `Objective-C Bridging Header: EngineBindings/generated/khiin_swift.h`
