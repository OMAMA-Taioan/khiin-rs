#!/bin/bash

set -e
WORK_DIR=$(dirname $0)
cd $WORK_DIR

PKG_NAME="khiin_swift"

# Build the project for the desired platforms:
cargo build --target x86_64-apple-darwin -p $PKG_NAME
cargo build --target aarch64-apple-darwin -p $PKG_NAME
mkdir -p ../../target/universal-macos/debug

lipo \
    ../../target/aarch64-apple-darwin/debug/lib$PKG_NAME.a \
    ../../target/x86_64-apple-darwin/debug/lib$PKG_NAME.a -create -output \
    ../../target/universal-macos/debug/lib$PKG_NAME.a

cargo build --target aarch64-apple-ios -p $PKG_NAME

cargo build --target x86_64-apple-ios -p $PKG_NAME
cargo build --target aarch64-apple-ios-sim -p $PKG_NAME
mkdir -p ../../target/universal-ios/debug

lipo \
    ../../target/aarch64-apple-ios-sim/debug/lib$PKG_NAME.a \
    ../../target/x86_64-apple-ios/debug/lib$PKG_NAME.a -create -output \
    ../../target/universal-ios/debug/lib$PKG_NAME.a
