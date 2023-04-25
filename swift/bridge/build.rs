use std::collections::HashMap;
use std::path::PathBuf;

use swift_bridge_build::ApplePlatform;
use swift_bridge_build::CreatePackageConfig;

fn main() {
    let out_dir = PathBuf::from("./generated");
    let bridges = vec!["src/lib.rs"];
    for path in &bridges {
        println!("cargo:rerun-if-changed={}", path);
    }

    swift_bridge_build::parse_bridges(bridges)
        .write_all_concatenated(out_dir.clone(), env!("CARGO_PKG_NAME"));

    swift_bridge_build::create_package(CreatePackageConfig {
        bridge_dir: out_dir,
        paths: HashMap::from([
            (
                ApplePlatform::IOS,
                "../../target/aarch64-apple-ios/debug/libkhiin_swift.a".into(),
            ),
            (
                ApplePlatform::Simulator,
                "../../target/universal-ios/debug/libkhiin_swift.a".into(),
            ),
            (
                ApplePlatform::MacOS,
                "../../target/universal-macos/debug/libkhiin_swift.a".into(),
            ),
        ]),
        out_dir: PathBuf::from("KhiinBridge"),
        package_name: String::from("KhiinBridge"),
    })
}
