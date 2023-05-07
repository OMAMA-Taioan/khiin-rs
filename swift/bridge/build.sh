#!/bin/bash

set -e
WORK_DIR=$(dirname $0)
cd $WORK_DIR

PKG_NAME="khiin_swift"

# IPHONEOS_DEPLOYMENT_TARGET=15.0
# MACOSX_DEPLOYMENT_TARGET=12.0

if [ "$RELEASE_MODE" = "release" ]; then
    BUILD_DIR="release"
    BUILD_FLAG="--release"
else
    BUILD_DIR="debug"
    BUILD_FLAG=""
fi

# Build the project for the desired platforms:
cargo build $BUILD_FLAG --target x86_64-apple-darwin -p $PKG_NAME
cargo build $BUILD_FLAG --target aarch64-apple-darwin -p $PKG_NAME
mkdir -p ../../target/universal-macos/$BUILD_DIR

lipo \
    ../../target/aarch64-apple-darwin/$BUILD_DIR/lib$PKG_NAME.a \
    ../../target/x86_64-apple-darwin/$BUILD_DIR/lib$PKG_NAME.a -create -output \
    ../../target/universal-macos/$BUILD_DIR/lib$PKG_NAME.a

cargo build $BUILD_FLAG --target aarch64-apple-ios -p $PKG_NAME

cargo build $BUILD_FLAG --target x86_64-apple-ios -p $PKG_NAME
cargo build $BUILD_FLAG --target aarch64-apple-ios-sim -p $PKG_NAME
mkdir -p ../../target/universal-ios/$BUILD_DIR

lipo \
    ../../target/aarch64-apple-ios-sim/$BUILD_DIR/lib$PKG_NAME.a \
    ../../target/x86_64-apple-ios/$BUILD_DIR/lib$PKG_NAME.a -create -output \
    ../../target/universal-ios/$BUILD_DIR/lib$PKG_NAME.a

swift-bridge-cli \
    create-package \
    --bridges-dir generated \
    --out-dir ../KhiinBridge \
    --ios \
        ../../target/aarch64-apple-ios/$BUILD_DIR/lib$PKG_NAME.a \
    --simulator \
        ../../target/universal-ios/$BUILD_DIR/lib$PKG_NAME.a \
    --macos \
        ../../target/universal-macos/$BUILD_DIR/lib$PKG_NAME.a \
    --name KhiinBridge
