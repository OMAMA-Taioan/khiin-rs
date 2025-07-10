$windowsDir = Split-Path -Path $PSScriptRoot -Parent
$workspaceDir = Split-Path -Path $windowsDir -Parent
$targetDir = "$workspaceDir\target"

$version = "0.1.3"
$x86toolchain = "i686-pc-windows-msvc"
$x64toolchain = "x86_64-pc-windows-msvc"
$tipDll = "khiin_windows.dll"
$svcExe = "khiin_service.exe"
$appExe = "khiin_helper.exe"
$dbFile = "$workspaceDir\resources\khiin.db"

$release = $args.Contains("--release")

Set-Location $workspaceDir

# cargo.exe make copy-db

if ($release) {
    cargo.exe build --release --target $x64toolchain --manifest-path=windows/ime/Cargo.toml
    cargo.exe build --release --target $x64toolchain --manifest-path=windows/service/Cargo.toml
    cargo.exe build --release --target $x86toolchain --manifest-path=windows/ime/Cargo.toml
    # cargo.exe build --release --target $x86toolchain --manifest-path=windows/service/Cargo.toml

    $wixOutDir = "$targetDir\release\wix"
    $x86TargetDir = "$targetDir\$x86toolchain\release"
    $x64TargetDir = "$targetDir\$x64toolchain\release"
    $targetDir = "$targetDir\release"
    $x64TipDllOutName = "Khiin PJH (64 bit).dll"
    $x86TipDllOutName = "Khiin PJH (32 bit).dll"
    $serviceOutName = "khiin_service.exe"
}
else {
    cargo.exe build --target $x64toolchain --manifest-path=windows/ime/Cargo.toml
    cargo.exe build --target $x64toolchain --manifest-path=windows/service/Cargo.toml
    cargo.exe build --target $x86toolchain --manifest-path=windows/ime/Cargo.toml
    # cargo.exe build --target $x86toolchain --manifest-path=windows/service/Cargo.toml

    $wixOutDir = "$targetDir\debug\wix"
    $x86TargetDir = "$targetDir\$x86toolchain\debug"
    $x64TargetDir = "$targetDir\$x64toolchain\debug"
    $targetDir = "$targetDir\debug"
    $x64TipDllOutName = "Khiin PJH (64 bit).dll"
    $x86TipDllOutName = "Khiin PJH (32 bit).dll"
    $serviceOutName = "khiin_service.exe"
}

$x64TipDll = "$x64TargetDir\$tipDll"
$x86TipDll = "$x86TargetDir\$tipDll"
$svcExe = "$x64TargetDir\$svcExe"
$appExe = "$workspaceDir\target\$x64toolchain\release\$appExe"
$appOutName = "khiin_helper.exe"
$licenseFileEN = "$PSScriptRoot\license.rtf"
$uiDialogBmp = "$PSScriptRoot\graphic-01.png"
$icon = "$PSScriptRoot\icon.ico"

Set-Location .\app\src-tauri
cargo.exe tauri build --target $x64toolchain
Set-Location $workspaceDir

$productWxs = "$PSScriptRoot\Product.wxs"
$registryWxs = "$PSScriptRoot\Registry.wxs"
$productObj = "$wixOutDir\Product.wixobj"
$registryObj = "$wixOutDir\Registry.wixobj"

$msiFilenameEn = "$wixOutDir\Khiin PJH v$version.msi"
$msiFilenameTw = "$wixOutDir\Khiin 打字法 v$version.msi"

candle.exe `
    -dWorkspaceDir="$workspaceDir" `
    -dSourceDir="$targetDir" `
    -dx86SourceDir="$x86TargetDir" `
    -dx64SourceDir="$x64TargetDir" `
    -dSource_TipDll32="$x86TipDll" `
    -dSource_TipDll64="$x64TipDll" `
    -dTarget_TipDll32="$x86TipDllOutName" `
    -dTarget_TipDll64="$x64TipDllOutName" `
    -dSource_ServiceExe="$svcExe" `
    -dTarget_ServiceExe="$serviceOutName" `
    -dSource_AppExe="$appExe" `
    -dTarget_AppExe="$appOutName" `
    -dSource_Database="$dbFile" `
    -dTarget_Database="khiin.db" `
    -dSource_LicenseRtf_EN="$licenseFileEN" `
    -dSource_Icon="$icon" `
    $productWxs `
    $registryWxs `
    -out "$wixOutDir\"
    
    # -dSource_UIDialogBmp="$uiDialogBmp" `


light.exe `
    $productObj `
    $registryObj `
    -cultures:en-us `
    -loc "$PSScriptRoot\en-us.wxl" `
    -ext WixUIExtension `
    -out "$msiFilenameEn"

    # light.exe `
    # $productObj `
    # $registryObj `
    # -cultures:zh-tw `
    # -loc "$PSScriptRoot\zh-tw.wxl" `
    # -ext WixUIExtension `
    # -out "$msiFilenameTw"
