// swift-tools-version: 5.8

import PackageDescription

let package = Package(
    name: "khiin-swift",
    defaultLocalization: "en",
    platforms: [
        .macOS("13.0")
    ],
    products: [
        .executable(
            name: "KhiinPJH",
            targets: ["KhiinPJH"]
        )
    ],
    dependencies: [
        .package(
            url: "https://github.com/apple/swift-protobuf.git",
            .upToNextMajor(from: "1.21.0")
        ),
        .package(
            url: "https://github.com/SwiftyBeaver/SwiftyBeaver.git",
            .upToNextMajor(from: "2.0.0")
        ),
        .package(
            path: "../KhiinBridge"
        ),
        .package(
            path: "../shared"
        )
    ],
    targets: [
        .executableTarget(
            name: "KhiinPJH",
            dependencies: [
                "KhiinBridge",
                "SwiftyBeaver",
                .product(
                    name: "SwiftProtobuf",
                    package: "swift-protobuf"
                ),
                .product(
                    name: "KhiinSwift",
                    package: "shared"
                )
            ],
            path: "src"
        )
    ]
)
