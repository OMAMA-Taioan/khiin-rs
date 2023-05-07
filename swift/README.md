# Khíín for iOS & macOS

Folder outline:

```
swift/
├── bridge/                     # Swift-Rust bridge module (rust)
│   └── Cargo.toml
├── ios/
│   ├── Keyboard/               # Keyboard app extension (`.appex` Bundle)
│   ├── Khiin/                  # Settings app (currently blank / for testing)
│   └── project.yml             # `xcodegen` project config
├── osx/                        # macOS input method swift package
│   └── Package.swift
└── shared/                     # Shared macOS/iOS code
    └── Package.swift
```

- `Khiin`: Currently a blank app with a text field simply for testing the IME
- `Keyboard`: The iOS Keyboard Extension (`.appex`) Bundle code
- `project.yml`: Builds the `.xcodeproj` for the iOS app
- `bridge`: A bridge module for Swift-Rust communication, using `swift-bridge`
- `osx`: The macOS Input Method
- `shared`: A Swift Package for shared code, mostly to operate the bridge

---

## Development

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

3. The following folders are generated during the build process:

```
swift/
├── bridge/
│   └── generated        # Rust bridge module binaries
├── ios/
│   └── Khiin.xcodeproj  # iOS app Xcode project
├── KhiinBridge          # Swift package for bridge module
├── osx/
│   ├── .build
│   └── src/
│       └── protos       # Protobuf generated files
└── shared/
    └── .build
```

These folders will be removed with `cargo make clean`.

---

## Rust-Swift Bridge Module

```bash
cargo make build-swift-bridge
```

The `bridge` module is compiled using
[`swift-bridge`](https://github.com/chinedufn/swift-bridge) into a `KhiinBridge`
Swift Package that can be consumed as a dependency. Building is a two-step
process.

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

---

## Khiin Keyboard (iOS)

_TODO: Repackage as a Swift Package like macOS app._

The XCode project must be built with `xcodegen`, it does not get checked in to
the repo. The command is simply `xcodegen` from the `swift` folder. Or `cargo
make xcodegen` from the top.

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

- The `Keyboard.appex` bundle will be embedded in the `Khiin.app` bundle for
  delivery onto the device.
- The iOS bundle identifier must start with something like `com.` or `org.`, and
  maybe a few others. However, we cannot use arbitrary bundle identifiers, as
  the original attempt to use `be.chiahpa` resulted in the IME not being
  recognized by the system.
- The `Khiin` package (the app) must have a `Settings.bundle` folder in order
  for the Keyboard extension to show up in system settings on iOS. The settings
  bundle does not need to have any properties or settings.

---

## KhiinPJH (macOS)

```bash
cargo make build-osx
```

The macOS input method is a normal OSX application. For an improved developer
experience, the input method is packaged for
[SPM](https://docs.swift.org/package-manager/PackageDescription/PackageDescription.html),
and a build script (`swift/osx/build.sh`) places all of the necessary items into
the `KhiinPJH.app` bundle folder, installed in `~/Library/Input Methods`. This
obviates the need for Xcode, and makes it possible to use other editors (namely
VSCode) for a better development experience.

The first time you run this command, you will need to log out and log back in to
activate the IME. During development, it is recommended to use the `watch`
version of the command, which will run in the background and update the package
every time you change the code.

```bash
cargo make build-osx # Run once first
cargo make watch-osx
```

Log output can be seen in another terminal with:

```bash
tail -f ~/Library/Caches/KhiinPJH/khiin_im.log
```

### macOS Notes

- The macOS Input Method bundle identifier **must** have `.inputmethod.` in the
  **3rd** position. That is: `a.b.inputmethod.c` is a valid bundle identifier.
  This is apparently not documented anywhere, but it was mentioned on some
  obscure websites and in old header files.
- The first time you build the IM (`cargo make build-osx`), you need to log out
  and log back in to refresh the available input methods. Subsequent builds
  should "just work", but YMMV. The IM will be available under the "Taioanese"
  language in the system settings.
- Resources:
  - https://github.com/GreenYun/Zhengma-macOS/wiki/003-info-plist
  - https://www.reddit.com/r/InputMethodKit/
  - https://github.com/pkamb/NumberInput_IMKit_Sample/issues/1
