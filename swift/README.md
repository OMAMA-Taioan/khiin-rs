# Khiin iOS & macOS

The rust `EngineBindings` can be built with `cargo-lipo` and `cbindgen`:

```
cargo install cargo-lipo
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
cargo lipo --targets aarch64-apple-ios-sim,x86_64-apple-ios
cbindgen --config cbindgen.toml --crate khiin_swift --output khiin_swift.h
```

This will produce the files `khiin-rs/target/universal/debug/khiin_swift.a` and
the `khiin_swift.h` header file.

Using `cargo lipo --release` with the same command will build the release
version.

In order to link to the `EngineBindings` target, other targets must add some
linker flags: Khiin Project > Target > Build Settings > Linking > Other Linker
Flags.

The debug and release flags should use the debug and release `khiin_swift.a` library files as appropriate:

Debug:
- -lkhiin_swift
- -L$(PROJECT_DIR)/../target/universal/debug

Release
- -lkhiin_swift
- L$(PROJECT_DIR)/../target/universal/release

