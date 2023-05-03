#!/bin/bash

set -e
work_dir=$(dirname $0)
cd $work_dir

build_dir=.build
build_output_dir=$build_dir/arm64-apple-macosx/debug
assets_dir=assets
bundle_dir=$build_dir/KhiinIM.app
contents_dir=$bundle_dir/Contents
bin_dir=$contents_dir/MacOS
res_dir=$contents_dir/Resources
icon_dir=$build_dir/AppIcon.iconset
im_dir="/Users/$(id -un)/Library/Input Methods/"
db_file=../../resources/khiin.db

# Prepare icon assets
rm -rf $icon_dir
mkdir -p $icon_dir
icon_src=$assets_dir/main_icon_512.png
sips -z 16 16     $icon_src --out $icon_dir/icon_16x16.png
sips -z 32 32     $icon_src --out $icon_dir/icon_16x16@2x.png
sips -z 32 32     $icon_src --out $icon_dir/icon_32x32.png
sips -z 64 64     $icon_src --out $icon_dir/icon_32x32@2x.png
sips -z 128 128   $icon_src --out $icon_dir/icon_128x128.png
sips -z 256 256   $icon_src --out $icon_dir/icon_128x128@2x.png
sips -z 256 256   $icon_src --out $icon_dir/icon_256x256.png
sips -z 512 512   $icon_src --out $icon_dir/icon_256x256@2x.png
cp                $icon_src       $icon_dir/icon_512x512.png
iconutil -c icns $icon_dir
icns_file=$build_dir/AppIcon.icns


# Build the application
swift build

# Bundle it into KhiinIM.app
rm -rf $bundle_dir
mkdir -p $bundle_dir
mkdir -p $contents_dir
mkdir -p $bin_dir
mkdir -p $res_dir
cp $build_output_dir/KhiinIM    $bin_dir
cp $assets_dir/Info.plist       $contents_dir
cp $assets_dir/PkgInfo          $contents_dir
cp $icns_file                   $res_dir
cp $assets_dir/main.svg         $res_dir
cp $assets_dir/menuicon.svg     $res_dir
cp -r $assets_dir/en.lproj      $res_dir
cp -r $assets_dir/zh-Hant.lproj $res_dir
cp $db_file                     $res_dir

# Move it to the user's input method folder
killall -9 KhiinIM || true
cp -r $bundle_dir "$im_dir"
ls -la "$im_dir"
echo "KhiinIM.app successfully installed to ~/Library/Input Methods"
echo "You may need to log out and in to see it in the System Settings."
