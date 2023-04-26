# Khíín for iOS & macOS

Folders:

- `Khiin`: Currently a blank app with a text field simply for testing the IME
- `Keyboard`: The iOS Keyboard Extension (`.appex`) Bundle code
- `KhiinIM`: The macOS Input Method
- `Protos`: The generated `.pb.swift` protobuf glass files
- `bridge`: A bridge module for Swift-Rust communication, using `swift-bridge`
  (Nb: we are currently using a custom fork until the changes get merged
  upstream)

The `Keyboard.appex` bundle will be embedded in the `Khiin.app` bundle for
delivery onto the device. 

## XCode

The XCode project must be built with `xcodegen`, it does not get checked in to
the repo. See instructions below.

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

## iOS Notes

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
- The `KhiinIM` package (macOS IME) needs to build its outputs into the
  `/Library/Input Methods` folder if you want to be able to test it. This should
  be configured already in the xcodeproj, but you must run `chmod 777
  /Library/Input\/ Methods` for it to work. Tip from
  [here](https://iosexample.com/inputmethodkit-sample-app-with-macos11-xcode13-swift5-5-in-2021/).
- I have not yet figured out a good way to rebuild the macOS target while logged
  in as a user and testing it. So far it seems like we need to log out, delete
  the old one, rebuild, then log out again to activate it. Hopefully there is a
  better way...
- Updates for the previous 2 points: the IME now builds using a build script, to
  avoid Xcode as much as possible because everything Xcode touches seems to fail
  miserably. The build script is in `swift/build-im.sh`, and it is rather
  ridiculous: it cleans everything, so there is never any caching. Then it
  builds the entire product into the `swift/.build` folder. Then it attempts to
  copy over the `KhiinIM.app` bundle to a test user account, which the developer
  should set up on their local machine, otherwise there is no good way to test
  other than logging in and out every time. To run the script, you currently
  need to `cd swift` and then run it manually, passing the name of the user
  account to install it to, e.g.: `sh build-im.sh -u testuser`. To test, the
  developer should then switch accounts (make an easy password and don't give
  sudo priviledges, you'll need to type this password **a lot**), test the IME,
  then **log out** (it is not enough to switch back, the testing user must fully
  log out, or updates will not be applied).
- We do not seem to be able to debug anything because Xcode gets in an infinite
  loop and crashes. However, we are able to see logs. To watch the logs while
  testing, run `tail -f ~/Library/Caches/KhiinIM/swiftybeaver.log` in your test
  user account before starting to test the IME.
- This is a horrendous developer workflow, painfully slow and susceptible to all
  kinds of manual errors. If anyone knows of a better way to do this, **please
  let me know**. Thank you.

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

For working on the macOS input method:

```bash
chmod 777 /Library\/ Input Methods
```

You are now ready to build the apps in XCode 😄

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

3. Files in `Protos` are generated using Apple's `swift-protobuf` plugin.

- `cbindgen`: a tool for creating the C header files used from Swift
- `swift-protobuf`: Apple's Protobuf extension for Swift

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

- `target/universal-ios/(debug|release)/libkhiin_swift.a` - for the simulator -
`target/aarch64-apple-ios/(debug|release)/libkhiin_swift.a` - for iOS devices -
`target/universal-macos/(debug|release)/libkhiin_swift.a` - for macOS devices

---

Instructions for recreating the "local swift package" setup for the bridge
module, from [here](https://archive.ph/tzWkB)

1. Build the actual `KhiinBridge` module using `cargo make`
1. Create a temporary local git repository
1. Paste the "file URL" for this local git repository into XCode
1. Drag the actual `KhiinBridge` folder into the XCode project, it will override
   the fake git repository
1. Delete the local git repository
1. Do not delete the reference to the (deleted/fake) local git repository in
   XCode

Here is a zsh/bash script (from the above URL) that accomplishes the above. Add
this to ~/.zshrc, then type `temppack KhiinBridge` (from any folder), drag the
real `KhiinBridge` folder over, then press `Y` at the command prompt.

```bash
temppack() {
  if [ -z "$1" ] 
    then
      echo "Usage: temppack PackageName"
  else
    mkdir temppack
    cd temppack 
    mkdir $1
    cd $1
    swift package init --type library
    git init
    git add .
    git commit -m "first"
    echo ""
    echo "file://$(pwd)" | pbcopy
    echo "The repository path for $1 is now in your clipboard."
    echo ""
    [[ $BASH_VERSION ]] && read -p "Delete temp files?[Y/N]" -n 1 -r
    [[ $ZSH_VERSION ]] && read -q "REPLY?Delete temp files?[Y/N]"
    echo ""
    cd ../..
    if [[ $REPLY =~ ^[Yy]$ ]]
    then
      echo "Deleting temporary folder..."
      rm -rf temppack 
      echo "Done."
    fi
  fi
}
```
