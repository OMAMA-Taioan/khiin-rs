#!/bin/bash

build_dir=.build
build_output_dir=$build_dir/arm64-apple-macosx/debug
contents_dir=$build_dir/KhiinIM.app/Contents
bin_dir=$build_dir/KhiinIM.app/Contents/MacOS
res_dir=$build_dir/KhiinIM.app/Contents/Resources

icon_dir=$build_dir/AppIcon.iconset
rm -rf $icon_dir
mkdir $icon_dir
icon_src=main_icon_512.png
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

rm -rf $build_dir/KhiinIM.app
mkdir -p $contents_dir
mkdir -p $bin_dir
mkdir -p $res_dir
cp $build_output_dir/KhiinIM $bin_dir
cp KhiinIM/Resources/Info.plist $contents_dir
cp PkgInfo $contents_dir
cp -r KhiinIM/Resources/en.lproj $res_dir
cp -r KhiinIM/Resources/zh-Hant.lproj $res_dir
cp KhiinIM/Resources/main.svg $res_dir
cp KhiinIM/Resources/menuicon.svg $res_dir
cp $icns_file $res_dir
cp ../resources/khiin.db $res_dir
