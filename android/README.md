# Khiin Android

## Development Environment

- Android Studio Flamingo
- Android NDK 25.2.9519653

Install the Android rust targets:

```bash
rustup target add armv7-linux-androideabi   # for arm
rustup target add i686-linux-android        # for x86
rustup target add aarch64-linux-android     # for arm64
rustup target add x86_64-linux-android      # for x86_64
```

Copy over the database file into the appropriate folder:

```bash
cargo install --force cargo-make    # Only the first time
cargo make build-droid
```

You don't need to copy the database again unless you make changes.

There is currently a
[bug](https://github.com/mozilla/rust-android-gradle/issues/105) that crashes
the Android app on an x86_64 emulator. It works on the x86 emulator, and on the
devices themselves. If the linked issue is ever fixed we will update to make
that work. For now, if you are using Windows, use an x86 emulator to run the
app. If you are using another platform (like macOS), you shouldn't need to worry
about this issue and can use the usual arm64 system images.

Note: The bug is listed in `rust-android-gradle`, even though we are using
[`android-rust`](https://github.com/MatrixDev/GradleAndroidRustPlugin). It's
related to the NDK and not specific to either plugin.
