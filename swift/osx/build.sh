#!/bin/bash

set -e
work_dir=$(dirname $0)
cd $work_dir

if [ "$RELEASE_MODE" = "release" ]; then
    BUILD_DIR="release"
    BUILD_FLAG="release"
else
    BUILD_DIR="debug"
    BUILD_FLAG="debug"
fi

app_name=KhiinPJH
build_dir=.build/artifacts/$BUILD_DIR
universal_dir=.build/universal-macosx/$BUILD_DIR
assets_dir=assets
bundle_dir=$build_dir/$app_name.app
contents_dir=$bundle_dir/Contents
bin_dir=$contents_dir/MacOS
res_dir=$contents_dir/Resources
app_dir=$contents_dir/Applications
icon_dir=.build/icons
iconset_dir=$icon_dir/AppIcon.iconset
im_dir="/Users/$(id -un)/Library/Input Methods/"
db_file=../../resources/khiin.db
helper_app=../../target/universal-apple-darwin/$BUILD_DIR/bundle/macos/khiin_helper.app

# Prepare icon assets
rm -rf $iconset_dir
mkdir -p $iconset_dir
icon_src=$assets_dir/main_icon_512.png
sips -z 16 16     $icon_src --out $iconset_dir/icon_16x16.png
sips -z 32 32     $icon_src --out $iconset_dir/icon_16x16@2x.png
sips -z 32 32     $icon_src --out $iconset_dir/icon_32x32.png
sips -z 64 64     $icon_src --out $iconset_dir/icon_32x32@2x.png
sips -z 128 128   $icon_src --out $iconset_dir/icon_128x128.png
sips -z 256 256   $icon_src --out $iconset_dir/icon_128x128@2x.png
sips -z 256 256   $icon_src --out $iconset_dir/icon_256x256.png
sips -z 512 512   $icon_src --out $iconset_dir/icon_256x256@2x.png
cp                $icon_src       $iconset_dir/icon_512x512.png
iconutil -c icns $iconset_dir
icns_file=$icon_dir/AppIcon.icns

# Build the application
swift build --configuration $BUILD_FLAG --triple x86_64-apple-macosx
swift build --configuration $BUILD_FLAG --triple arm64-apple-macosx
mkdir -p $universal_dir
lipo \
    .build/arm64-apple-macosx/$BUILD_DIR/$app_name \
    .build/x86_64-apple-macosx/$BUILD_DIR/$app_name -create -output \
    $universal_dir/$app_name

# Bundle it into .app
rm -rf $bundle_dir
mkdir -p $bundle_dir
mkdir -p $contents_dir
mkdir -p $bin_dir
mkdir -p $res_dir
mkdir -p $app_dir
cp $universal_dir/$app_name         $bin_dir
cp $assets_dir/Info.plist           $contents_dir
cp $assets_dir/PkgInfo              $contents_dir
cp $icns_file                       $res_dir
cp $assets_dir/main.svg             $res_dir
cp $assets_dir/menuicon.svg         $res_dir
cp -r $assets_dir/en.lproj          $res_dir
cp -r $assets_dir/zh-Hant.lproj     $res_dir
cp $db_file                         $res_dir
if [ -d $helper_app ]; then
    cp -r $helper_app $app_dir
else
    echo "warning: please build helper_app $helper_app"
fi
# Move it to the user's input method folder
killall -9 $app_name || true
cp -r $bundle_dir "$im_dir"
ls -la "$im_dir"
echo "$app_name.app successfully installed to ~/Library/Input Methods"
echo "You may need to log out and in to see it in the System Settings."

pkgbuild \
    --info assets/PackageInfo \
    --root $build_dir/KhiinPJH.app \
    --identifier app.khiin.inputmethod.khiin \
    --version "0.2.5" \
    --install-location "/tmp/KhiinPJH.app" \
    --scripts assets/scripts \
    "$build_dir/KhiinPJH.pkg"
