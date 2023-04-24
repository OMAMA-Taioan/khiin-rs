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

Note that you will also see a warning when building the project:

```
Linking against a dylib which is not safe for use in application extensions:
/Users/.../SwiftProtobuf...
```

I have not yet figured out how to fix this warning, although it does not seem to
prevent any problems and the app runs as expected.

## Development Environment

1. Use `cargo-make` for the commands listed in this document:

```
cargo install --force cargo-make
```

2. You **must** build and copy over the database for the app to run:

```
cargo make db-copy-mac
```

3. Files in the `EngineBindings/generated` file are automatically generated as
explained in this README. If you are not modifying the rust or protobuf code,
you should not need to modify these files. If you do intend to make
modifications, then you must install the dependencies and regenerate the files.
The dependencies are:

- `cargo-lipo`: a cargo plugin for building mac/iOS universal binaries
- `cbindgen`: a tool for creating the C header files used from Swift
- `swift-protobuf`: Apple's Protobuf extension for Swift

These may be installed as follows:

```
cargo install cargo-lipo
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
cargo install --force cbindgen
brew install swift-protobuf
```

Once everything is done, you must run `cargo make` to prepare everything for
building the mac/iOS apps. You should see an output similar to the following:

```
$ cargo make
[cargo-make] INFO - cargo make 0.36.6
[cargo-make] INFO - Calling cargo metadata to extract project info
[cargo-make] INFO - Cargo metadata done
[cargo-make] INFO - Build File: Makefile.toml
[cargo-make] INFO - Task: default
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Execute Command: "cargo" "lipo" "--targets" "aarch64-apple-ios-sim,x86_64-apple-ios"
[INFO  cargo_lipo::meta] Will build universal library for ["khiin_swift"]
[INFO  cargo_lipo::lipo] Building "khiin_swift" for "aarch64-apple-ios-sim"
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
[INFO  cargo_lipo::lipo] Building "khiin_swift" for "x86_64-apple-ios"
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
[INFO  cargo_lipo::lipo] Creating universal library for khiin_swift
[cargo-make] INFO - Running Task: mac-builddirs
[cargo-make] INFO - Execute Command: "cbindgen" "--config" "swift/EngineBindings/cbindgen.toml" "--crate" "khiin_swift" "--output" "swift/EngineBindings/generated/khiin_swift.h"
[cargo-make] INFO - Running Task: build-mac-protos
[cargo-make] INFO - Execute Command: "python3" "src/sql_gen.py" "-f" "data/frequency.csv" "-c" "data/conversions_all.csv" "-s" "data/syllables.txt" "-t" "-y" "data/symbols.tsv" "-e" "data/emoji.csv" "-o" "out/khiin_db.sql" "-d" "out/khiin.db"
Building database, please wait...Output written to out/khiin_db.sql:
 - 12242 inputs ("frequency" table)
 - 25403 tokens ("conversions" table)
 - 1514 syllables ("syllables" table)
[cargo-make] INFO - Running Task: db-copy
[cargo-make] INFO - Running Task: db-copy-mac
[cargo-make] INFO - Running Task: db-copy-to-target
[cargo-make] INFO - Execute Command: "cargo" "build" "--manifest-path=cli/Cargo.toml"
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
[cargo-make] INFO - Build Done in 4.89 seconds.
```

Using `cargo lipo --release` with the same command will build the release
version.

### XCode linker configuration

This configuration should already be done, so you shouldn't need to change it
unless you are editing the builds.

In order to link to the `EngineBindings` target, other targets must add some
linker flags:

- Khiin Project > Target > Build Settings > Linking > Other Linker Flags

The debug and release flags should use the debug and release `khiin_swift.a` library files as appropriate:

Debug:
- -lkhiin_swift
- -L$(PROJECT_DIR)/../target/universal/debug

Release
- -lkhiin_swift
- L$(PROJECT_DIR)/../target/universal/release

Also under the Build Settings > Swift Compiler - General section, you must set
the bridging header:

- `Objective-C Bridging Header: EngineBindings/generated/khiin_swift.h`
